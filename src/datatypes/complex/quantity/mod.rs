//! The FHIR `Quantity` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Quantity,
//! hl7.org/fhir/R5/datatypes-definitions.html#Quantity.code)
//! - Fields: `value` (0..1, `decimal`), `comparator` (0..1, `code` — one of `<`, `<=`,
//!   `>=`, `>`, `ad`, qualifying how `value` should be understood; this crate validates
//!   `code` grammar only, not the `ValueSet` binding, matching every other bound `code`
//!   field in this crate), `unit` (0..1, `string` — human-readable unit), `system`
//!   (0..1, `uri` — the unit code system, e.g. UCUM), `code` (0..1, `code` —
//!   computer-processable unit in `system`).
//! - Invariant `qty-3` (FHIRPath: `code.empty() or system.exists()`, **error**
//!   severity): "If a code for the unit is present, the system SHALL also be present."
//!   Implementable and enforced here (unlike `Period`'s `per-1`/`Coding`'s `cod-1`) —
//!   it's a plain presence check, no boundary/comparison semantics needed.
//! - Like every FHIR type, `Quantity` inherits `id` (0..1, `string`) and `extension`
//!   (0..*, `Extension`) from `Element`, and is therefore itself subject to `ele-1`
//!   ("must have a value or children") — with no *primitive* `value` attribute of its
//!   own in the FHIRPath sense (its `value` field is just one of several children),
//!   at least one of `value`/`comparator`/`unit`/`system`/`code`/`extension` must be
//!   present.
//! - Every value field is primitive-valued, so each is
//!   [`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern (see `Extension`'s module docs). `Quantity.id` is a plain ordinary
//!   property, never itself companion-wrapped, matching `Extension.id`/`Period.id`/
//!   `Coding.id`.
//!
//! # Usage
//! Use [`Quantity::new`] to construct a validated instance (checks `ele-1` and
//! `qty-3`), or [`Quantity::new_unchecked`] when the fields are already known to
//! satisfy them.

#[cfg(feature = "serde")]
use serde::de::{Error as DeError, MapAccess, Visitor};
#[cfg(feature = "serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::datatypes::complex::Extension;
use crate::datatypes::primitive::Primitive;
#[cfg(feature = "serde")]
use crate::datatypes::primitive::{
    PrimitiveCompanion, merge_primitive_entry, serialize_primitive_entry,
};
use crate::errors::{FhirCoreResult, constraints::ConstraintError};
use crate::types::{Code, Decimal, FhirString, Uri};

#[cfg(test)]
mod test;

/// The FHIR `Quantity` complex data type: a measured amount with an optional unit,
/// unit-coding system, and comparator for open-ended/approximate values.
///
/// # Invariants
/// Any instance of `Quantity` constructed via [`Quantity::new`] is guaranteed to
/// satisfy:
/// - `ele-1`: at least one of `value`, `comparator`, `unit`, `system`, `code`, or
///   `extension` is set.
/// - `qty-3`: if `code` is set, `system` is also set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quantity {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    value: Option<Primitive<Decimal>>,
    comparator: Option<Primitive<Code>>,
    unit: Option<Primitive<FhirString>>,
    system: Option<Primitive<Uri>>,
    code: Option<Primitive<Code>>,
}

#[cfg(feature = "serde")]
impl Serialize for Quantity {
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
        if let Some(value) = &self.value {
            serialize_primitive_entry(&mut map, "value", "_value", value)?;
        }
        if let Some(comparator) = &self.comparator {
            serialize_primitive_entry(&mut map, "comparator", "_comparator", comparator)?;
        }
        if let Some(unit) = &self.unit {
            serialize_primitive_entry(&mut map, "unit", "_unit", unit)?;
        }
        if let Some(system) = &self.system {
            serialize_primitive_entry(&mut map, "system", "_system", system)?;
        }
        if let Some(code) = &self.code {
            serialize_primitive_entry(&mut map, "code", "_code", code)?;
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
impl<'de> Deserialize<'de> for Quantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QuantityVisitor;

        impl<'de> Visitor<'de> for QuantityVisitor {
            type Value = Quantity;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FHIR Quantity JSON object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Quantity, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut extension = Vec::new();
                let mut value: Slot<Decimal> = Slot::default();
                let mut comparator: Slot<Code> = Slot::default();
                let mut unit: Slot<FhirString> = Slot::default();
                let mut system: Slot<Uri> = Slot::default();
                let mut code: Slot<Code> = Slot::default();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => id = Some(map.next_value()?),
                        "extension" => extension = map.next_value()?,
                        "value" => value.value = Some(map.next_value()?),
                        "_value" => value.companion = Some(map.next_value()?),
                        "comparator" => comparator.value = Some(map.next_value()?),
                        "_comparator" => comparator.companion = Some(map.next_value()?),
                        "unit" => unit.value = Some(map.next_value()?),
                        "_unit" => unit.companion = Some(map.next_value()?),
                        "system" => system.value = Some(map.next_value()?),
                        "_system" => system.companion = Some(map.next_value()?),
                        "code" => code.value = Some(map.next_value()?),
                        "_code" => code.companion = Some(map.next_value()?),
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let value = value
                    .is_present()
                    .then(|| merge_primitive_entry(value.value, value.companion))
                    .transpose()?;
                let comparator = comparator
                    .is_present()
                    .then(|| merge_primitive_entry(comparator.value, comparator.companion))
                    .transpose()?;
                let unit = unit
                    .is_present()
                    .then(|| merge_primitive_entry(unit.value, unit.companion))
                    .transpose()?;
                let system = system
                    .is_present()
                    .then(|| merge_primitive_entry(system.value, system.companion))
                    .transpose()?;
                let code = code
                    .is_present()
                    .then(|| merge_primitive_entry(code.value, code.companion))
                    .transpose()?;

                Quantity::new(value, comparator, unit, system, code, id, extension)
                    .map_err(DeError::custom)
            }
        }

        deserializer.deserialize_map(QuantityVisitor)
    }
}

impl Quantity {
    /// Creates a new `Quantity`, validating the `ele-1` and `qty-3` invariants.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Constraint`](crate::errors::FhirCoreError::Constraint)
    /// containing [`ConstraintError::InvariantViolated`] if:
    /// - `value`, `comparator`, `unit`, `system`, `code`, and `extension` are all
    ///   absent/empty (`ele-1`), or
    /// - `code` is present but `system` is absent (`qty-3`).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::datatypes::complex::quantity::Quantity;
    /// use fhir_core::datatypes::primitive::Primitive;
    /// use fhir_core::types::{Code, Decimal, Uri};
    ///
    /// let value = Primitive::from_value(Decimal::try_from("5.4").unwrap());
    /// let system = Primitive::from_value(Uri::new("http://unitsofmeasure.org").unwrap());
    /// let code = Primitive::from_value(Code::new("mg").unwrap());
    /// let quantity = Quantity::new(Some(value), None, None, Some(system), Some(code), None, Vec::new());
    /// assert!(quantity.is_ok());
    ///
    /// // code without system violates qty-3.
    /// let code_only = Quantity::new(
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     Some(Primitive::from_value(Code::new("mg").unwrap())),
    ///     None,
    ///     Vec::new(),
    /// );
    /// assert!(code_only.is_err());
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        value: Option<Primitive<Decimal>>,
        comparator: Option<Primitive<Code>>,
        unit: Option<Primitive<FhirString>>,
        system: Option<Primitive<Uri>>,
        code: Option<Primitive<Code>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> FhirCoreResult<Self> {
        Self::validate_ele1(&value, &comparator, &unit, &system, &code, &extension)?;
        Self::validate_qty3(&system, &code)?;
        Ok(Self {
            id,
            extension,
            value,
            comparator,
            unit,
            system,
            code,
        })
    }

    /// Validates the `ele-1` invariant against candidate fields.
    #[allow(clippy::too_many_arguments)]
    fn validate_ele1(
        value: &Option<Primitive<Decimal>>,
        comparator: &Option<Primitive<Code>>,
        unit: &Option<Primitive<FhirString>>,
        system: &Option<Primitive<Uri>>,
        code: &Option<Primitive<Code>>,
        extension: &[Extension],
    ) -> Result<(), ConstraintError> {
        if value.is_none()
            && comparator.is_none()
            && unit.is_none()
            && system.is_none()
            && code.is_none()
            && extension.is_empty()
        {
            return Err(ConstraintError::InvariantViolated {
                key: "ele-1",
                description: "All FHIR elements must have a @value or children".to_owned(),
            });
        }
        Ok(())
    }

    /// Validates the `qty-3` invariant: if `code` is present, `system` must be too.
    fn validate_qty3(
        system: &Option<Primitive<Uri>>,
        code: &Option<Primitive<Code>>,
    ) -> Result<(), ConstraintError> {
        if code.is_some() && system.is_none() {
            return Err(ConstraintError::InvariantViolated {
                key: "qty-3",
                description: "If a code for the unit is present, the system SHALL also be present"
                    .to_owned(),
            });
        }
        Ok(())
    }

    /// Creates a new `Quantity` without validating the `ele-1`/`qty-3` invariants.
    ///
    /// # Warning
    /// The caller is responsible for ensuring the fields satisfy both invariants.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new_unchecked(
        value: Option<Primitive<Decimal>>,
        comparator: Option<Primitive<Code>>,
        unit: Option<Primitive<FhirString>>,
        system: Option<Primitive<Uri>>,
        code: Option<Primitive<Code>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> Self {
        Self {
            id,
            extension,
            value,
            comparator,
            unit,
            system,
            code,
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

    /// Returns the measured value, if present.
    #[inline]
    pub fn value(&self) -> Option<&Primitive<Decimal>> {
        self.value.as_ref()
    }

    /// Returns the comparator, if present.
    #[inline]
    pub fn comparator(&self) -> Option<&Primitive<Code>> {
        self.comparator.as_ref()
    }

    /// Returns the human-readable unit, if present.
    #[inline]
    pub fn unit(&self) -> Option<&Primitive<FhirString>> {
        self.unit.as_ref()
    }

    /// Returns the unit code system, if present.
    #[inline]
    pub fn system(&self) -> Option<&Primitive<Uri>> {
        self.system.as_ref()
    }

    /// Returns the coded unit, if present.
    #[inline]
    pub fn code(&self) -> Option<&Primitive<Code>> {
        self.code.as_ref()
    }
}
