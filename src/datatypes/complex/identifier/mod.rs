//! The FHIR `Identifier` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Identifier,
//! hl7.org/fhir/R5/datatypes-definitions.html#Identifier.assigner)
//! - Fields: `use` (0..1, `code` — usual | official | temp | secondary | old),
//!   `type` (0..1, [`CodeableConcept`] — describes the identifier's kind, e.g. MRN),
//!   `system` (0..1, `uri` — the namespace the value is unique within), `value` (0..1,
//!   `string`), `period` (0..1, [`Period`] — time period the identifier is valid for),
//!   `assigner` (0..1, [`Reference`] to `Organization` — who issued it).
//! - Invariant `ident-1` (FHIRPath: `value.exists()`, **Warning** severity): "Identifier
//!   with no value has limited utility." Not enforced here — same reasoning as
//!   `Coding`'s `cod-1`: this crate's `ConstraintError` models hard SHALL failures, and
//!   `ident-1` is a SHOULD, not a SHALL. Only `ele-1` is validated.
//! - Like every FHIR type, `Identifier` inherits `id` (0..1, `string`) and `extension`
//!   (0..*, `Extension`) from `Element`, and is therefore itself subject to `ele-1`
//!   ("must have a value or children") — at least one of `use`/`type`/`system`/
//!   `value`/`period`/`assigner`/`extension` must be present.
//! - `use`, `system`, `value` are primitive-valued
//!   ([`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern); `type`, `period`, `assigner` are complex types and get no companion.
//!   `Identifier.id` is a plain ordinary property, never itself companion-wrapped,
//!   matching every other type in this crate.
//! - `use` is a Rust keyword — kept as the raw identifier `r#use` to preserve the exact
//!   FHIR property name, matching this crate's existing convention for the `type` field
//!   (see e.g. `TypeError::r#type` in `src/errors/type.rs`).
//! - `assigner: Box<Reference>` — boxed because [`Reference`] embeds
//!   [`Identifier`] right back (`Reference.identifier`), and two directly-nested structs
//!   referencing each other by value would be infinite-sized. `Reference` mirrors this
//!   with `Box<Identifier>`.
//!
//! # Usage
//! Use [`Identifier::new`] to construct a validated instance (checks `ele-1`), or
//! [`Identifier::new_unchecked`] when the fields are already known to satisfy it.

#[cfg(feature = "serde")]
use serde::de::{Error as DeError, MapAccess, Visitor};
#[cfg(feature = "serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::codeable_concept::CodeableConcept;
use crate::datatypes::complex::period::Period;
use crate::datatypes::complex::reference::Reference;
use crate::datatypes::primitive::Primitive;
#[cfg(feature = "serde")]
use crate::datatypes::primitive::{
    PrimitiveCompanion, merge_primitive_entry, serialize_primitive_entry,
};
use crate::errors::{FhirCoreResult, constraints::ConstraintError};
use crate::types::{Code, FhirString, Uri};

#[cfg(test)]
mod test;

/// The FHIR `Identifier` complex data type: a business identifier for a resource,
/// scoped to a namespace (`system`) and optionally typed, time-bounded, and attributed
/// to an issuing organization.
///
/// # Invariants
/// Any instance of `Identifier` constructed via [`Identifier::new`] is guaranteed to
/// satisfy `ele-1`: at least one of `use`, `type`, `system`, `value`, `period`,
/// `assigner`, or `extension` is set. See the module docs for why `ident-1` (a
/// warning-severity invariant) is not validated here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    r#use: Option<Primitive<Code>>,
    r#type: Option<CodeableConcept>,
    system: Option<Primitive<Uri>>,
    value: Option<Primitive<FhirString>>,
    period: Option<Period>,
    assigner: Option<Box<Reference>>,
}

#[cfg(feature = "serde")]
impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        if let Some(id) = &self.id {
            map.serialize_entry("id", id)?;
        }
        if !self.extension.is_empty() {
            map.serialize_entry("extension", &self.extension)?;
        }
        if let Some(r#use) = &self.r#use {
            serialize_primitive_entry(&mut map, "use", "_use", r#use)?;
        }
        if let Some(r#type) = &self.r#type {
            map.serialize_entry("type", r#type)?;
        }
        if let Some(system) = &self.system {
            serialize_primitive_entry(&mut map, "system", "_system", system)?;
        }
        if let Some(value) = &self.value {
            serialize_primitive_entry(&mut map, "value", "_value", value)?;
        }
        if let Some(period) = &self.period {
            map.serialize_entry("period", period)?;
        }
        if let Some(assigner) = &self.assigner {
            map.serialize_entry("assigner", assigner)?;
        }
        map.end()
    }
}

/// Per-type bare-value/companion accumulator pair used while scanning the JSON map. See
/// `Extension`'s `ValueSlot` for the fuller explanation this mirrors.
#[cfg(feature = "serde")]
struct Slot<T> {
    value: Option<T>,
    companion: Option<PrimitiveCompanion>,
}

// Manual impl: `#[derive(Default)]` would require `T: Default`, which none of these
// primitive types implement (and shouldn't need to).
#[cfg(feature = "serde")]
impl<T> Default for Slot<T> {
    fn default() -> Self {
        Self {
            value: None,
            companion: None,
        }
    }
}

#[cfg(feature = "serde")]
impl<T> Slot<T> {
    fn is_present(&self) -> bool {
        self.value.is_some() || self.companion.is_some()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdentifierVisitor;

        impl<'de> Visitor<'de> for IdentifierVisitor {
            type Value = Identifier;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FHIR Identifier JSON object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Identifier, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut extension = Vec::new();
                let mut r#use: Slot<Code> = Slot::default();
                let mut r#type: Option<CodeableConcept> = None;
                let mut system: Slot<Uri> = Slot::default();
                let mut value: Slot<FhirString> = Slot::default();
                let mut period: Option<Period> = None;
                let mut assigner: Option<Box<Reference>> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => id = Some(map.next_value()?),
                        "extension" => extension = map.next_value()?,
                        "use" => r#use.value = Some(map.next_value()?),
                        "_use" => r#use.companion = Some(map.next_value()?),
                        "type" => r#type = Some(map.next_value()?),
                        "system" => system.value = Some(map.next_value()?),
                        "_system" => system.companion = Some(map.next_value()?),
                        "value" => value.value = Some(map.next_value()?),
                        "_value" => value.companion = Some(map.next_value()?),
                        "period" => period = Some(map.next_value()?),
                        "assigner" => assigner = Some(map.next_value()?),
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let r#use = r#use
                    .is_present()
                    .then(|| merge_primitive_entry(r#use.value, r#use.companion))
                    .transpose()?;
                let system = system
                    .is_present()
                    .then(|| merge_primitive_entry(system.value, system.companion))
                    .transpose()?;
                let value = value
                    .is_present()
                    .then(|| merge_primitive_entry(value.value, value.companion))
                    .transpose()?;

                Identifier::new(
                    r#use, r#type, system, value, period, assigner, id, extension,
                )
                .map_err(DeError::custom)
            }
        }

        deserializer.deserialize_map(IdentifierVisitor)
    }
}

impl Identifier {
    /// Creates a new `Identifier`, validating the `ele-1` invariant.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Constraint`](crate::errors::FhirCoreError::Constraint)
    /// containing [`ConstraintError::InvariantViolated`] if `use`, `type`, `system`,
    /// `value`, `period`, `assigner`, and `extension` are all absent/empty.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::datatypes::complex::identifier::Identifier;
    /// use fhir_core::datatypes::primitive::Primitive;
    /// use fhir_core::types::FhirString;
    ///
    /// let value = Primitive::from_value(FhirString::new("12345").unwrap());
    /// let identifier = Identifier::new(None, None, None, Some(value), None, None, None, Vec::new());
    /// assert!(identifier.is_ok());
    ///
    /// let empty = Identifier::new(None, None, None, None, None, None, None, Vec::new());
    /// assert!(empty.is_err());
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        r#use: Option<Primitive<Code>>,
        r#type: Option<CodeableConcept>,
        system: Option<Primitive<Uri>>,
        value: Option<Primitive<FhirString>>,
        period: Option<Period>,
        assigner: Option<Box<Reference>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> FhirCoreResult<Self> {
        Self::validate_ele1(
            &r#use, &r#type, &system, &value, &period, &assigner, &extension,
        )?;
        Ok(Self {
            id,
            extension,
            r#use,
            r#type,
            system,
            value,
            period,
            assigner,
        })
    }

    /// Validates the `ele-1` invariant against candidate fields.
    #[allow(clippy::too_many_arguments)]
    fn validate_ele1(
        r#use: &Option<Primitive<Code>>,
        r#type: &Option<CodeableConcept>,
        system: &Option<Primitive<Uri>>,
        value: &Option<Primitive<FhirString>>,
        period: &Option<Period>,
        assigner: &Option<Box<Reference>>,
        extension: &[Extension],
    ) -> Result<(), ConstraintError> {
        if r#use.is_none()
            && r#type.is_none()
            && system.is_none()
            && value.is_none()
            && period.is_none()
            && assigner.is_none()
            && extension.is_empty()
        {
            return Err(ConstraintError::InvariantViolated {
                key: "ele-1",
                description: "All FHIR elements must have a @value or children".to_owned(),
            });
        }
        Ok(())
    }

    /// Creates a new `Identifier` without validating the `ele-1` invariant.
    ///
    /// # Warning
    /// The caller is responsible for ensuring at least one field satisfies `ele-1`.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new_unchecked(
        r#use: Option<Primitive<Code>>,
        r#type: Option<CodeableConcept>,
        system: Option<Primitive<Uri>>,
        value: Option<Primitive<FhirString>>,
        period: Option<Period>,
        assigner: Option<Box<Reference>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> Self {
        Self {
            id,
            extension,
            r#use,
            r#type,
            system,
            value,
            period,
            assigner,
        }
    }

    /// Returns the element-level identifier, if present.
    #[inline]
    pub fn id(&self) -> Option<&FhirString> {
        self.id.as_ref()
    }

    /// Returns the slice of nested extensions.
    #[inline]
    pub fn extensions(&self) -> &[Extension] {
        &self.extension
    }

    /// Returns the identifier's use, if present.
    #[inline]
    pub fn r#use(&self) -> Option<&Primitive<Code>> {
        self.r#use.as_ref()
    }

    /// Returns the identifier's type, if present.
    #[inline]
    pub fn r#type(&self) -> Option<&CodeableConcept> {
        self.r#type.as_ref()
    }

    /// Returns the namespace the value is unique within, if present.
    #[inline]
    pub fn system(&self) -> Option<&Primitive<Uri>> {
        self.system.as_ref()
    }

    /// Returns the identifier value, if present.
    #[inline]
    pub fn value(&self) -> Option<&Primitive<FhirString>> {
        self.value.as_ref()
    }

    /// Returns the time period the identifier is valid for, if present.
    #[inline]
    pub fn period(&self) -> Option<&Period> {
        self.period.as_ref()
    }

    /// Returns the organization that issued the identifier, if present.
    #[inline]
    pub fn assigner(&self) -> Option<&Reference> {
        self.assigner.as_deref()
    }
}
