//! The FHIR `Reference` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/references.html#Reference)
//! - Fields: `reference` (0..1, `string` — a literal reference, e.g.
//!   `"Patient/123"` or a contained `"#id"`), `type` (0..1, `uri` — the expected target
//!   resource type), `identifier` (0..1, [`Identifier`] — used when no literal
//!   reference is available), `display` (0..1, `string` — plain text naming the
//!   target).
//! - Two named invariants, both **error** severity:
//!   - `ref-2`: `reference.exists() or identifier.exists() or display.exists() or
//!     extension.exists()` — at least one of the four must be present. Note `type` is
//!     *not* in this list. Implemented in [`Reference::new`], same shape as
//!     `Extension::validate_ext1`. This is strictly stronger than the generic `ele-1`
//!     every `Element` carries (which would also accept `type`-only) — enforcing
//!     `ref-2` makes a separate `ele-1` check redundant, so only `ref-2` is validated.
//!   - `ref-1` (**not implementable here**): `reference.exists() implies
//!     (reference.startsWith('#').not() or (reference.substring(1) in
//!     %rootResource.contained.id) or ...)` — validating a local (`#id`) reference
//!     requires whole-resource-tree context (`%rootResource`/`%resource`,
//!     `contained`) this crate doesn't model (no `Resource`/`contained` support yet —
//!     see the `Base`/`Element`/`Resource` item in `docs/BACKLOG.md`'s roadmap). A
//!     `Reference` with a `"#id"` value is accepted here without checking that `#id`
//!     actually resolves to a sibling `contained` resource; don't confuse "constructs"
//!     with "spec-valid in full resource context."
//! - `reference`, `type`, `display` are primitive-valued
//!   ([`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern); `identifier` is a complex type and gets no companion.
//!   `Reference.id` is a plain ordinary property, never itself companion-wrapped,
//!   matching every other type in this crate.
//! - `identifier: Box<Identifier>` — boxed for the same mutual-embedding reason
//!   documented on [`Identifier`]'s `assigner` field.
//!
//! # Usage
//! Use [`Reference::new`] to construct a validated instance (checks `ref-2`), or
//! [`Reference::new_unchecked`] when the fields are already known to satisfy it.

#[cfg(feature = "serde")]
use serde::de::{Error as DeError, MapAccess, Visitor};
#[cfg(feature = "serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::identifier::Identifier;
use crate::datatypes::primitive::Primitive;
#[cfg(feature = "serde")]
use crate::datatypes::primitive::{
    PrimitiveCompanion, merge_primitive_entry, serialize_primitive_entry,
};
use crate::errors::{FhirCoreResult, constraints::ConstraintError};
use crate::types::{FhirString, Uri};

#[cfg(test)]
mod test;

/// The FHIR `Reference` complex data type: a pointer to another resource, by literal
/// reference string, business identifier, or display text (at least one of the three,
/// or an extension, per `ref-2`).
///
/// # Invariants
/// Any instance of `Reference` constructed via [`Reference::new`] is guaranteed to
/// satisfy `ref-2`: at least one of `reference`, `identifier`, `display`, or
/// `extension` is set (`type` alone does not count). See the module docs for why
/// `ref-1` is not validated here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    reference: Option<Primitive<FhirString>>,
    r#type: Option<Primitive<Uri>>,
    identifier: Option<Box<Identifier>>,
    display: Option<Primitive<FhirString>>,
}

#[cfg(feature = "serde")]
impl Serialize for Reference {
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
        if let Some(reference) = &self.reference {
            serialize_primitive_entry(&mut map, "reference", "_reference", reference)?;
        }
        if let Some(r#type) = &self.r#type {
            serialize_primitive_entry(&mut map, "type", "_type", r#type)?;
        }
        if let Some(identifier) = &self.identifier {
            map.serialize_entry("identifier", identifier)?;
        }
        if let Some(display) = &self.display {
            serialize_primitive_entry(&mut map, "display", "_display", display)?;
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
impl<'de> Deserialize<'de> for Reference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReferenceVisitor;

        impl<'de> Visitor<'de> for ReferenceVisitor {
            type Value = Reference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FHIR Reference JSON object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Reference, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut extension = Vec::new();
                let mut reference: Slot<FhirString> = Slot::default();
                let mut r#type: Slot<Uri> = Slot::default();
                let mut identifier: Option<Box<Identifier>> = None;
                let mut display: Slot<FhirString> = Slot::default();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => id = Some(map.next_value()?),
                        "extension" => extension = map.next_value()?,
                        "reference" => reference.value = Some(map.next_value()?),
                        "_reference" => reference.companion = Some(map.next_value()?),
                        "type" => r#type.value = Some(map.next_value()?),
                        "_type" => r#type.companion = Some(map.next_value()?),
                        "identifier" => identifier = Some(map.next_value()?),
                        "display" => display.value = Some(map.next_value()?),
                        "_display" => display.companion = Some(map.next_value()?),
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let reference = reference
                    .is_present()
                    .then(|| merge_primitive_entry(reference.value, reference.companion))
                    .transpose()?;
                let r#type = r#type
                    .is_present()
                    .then(|| merge_primitive_entry(r#type.value, r#type.companion))
                    .transpose()?;
                let display = display
                    .is_present()
                    .then(|| merge_primitive_entry(display.value, display.companion))
                    .transpose()?;

                Reference::new(reference, r#type, identifier, display, id, extension)
                    .map_err(DeError::custom)
            }
        }

        deserializer.deserialize_map(ReferenceVisitor)
    }
}

impl Reference {
    /// Creates a new `Reference`, validating the `ref-2` invariant.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Constraint`](crate::errors::FhirCoreError::Constraint)
    /// containing [`ConstraintError::InvariantViolated`] if `reference`, `identifier`,
    /// `display`, and `extension` are all absent/empty (`type` alone does not satisfy
    /// `ref-2`).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::datatypes::complex::reference::Reference;
    /// use fhir_core::datatypes::primitive::Primitive;
    /// use fhir_core::types::FhirString;
    ///
    /// let reference = Primitive::from_value(FhirString::new("Patient/123").unwrap());
    /// let r = Reference::new(Some(reference), None, None, None, None, Vec::new());
    /// assert!(r.is_ok());
    ///
    /// let empty = Reference::new(None, None, None, None, None, Vec::new());
    /// assert!(empty.is_err());
    /// ```
    pub fn new(
        reference: Option<Primitive<FhirString>>,
        r#type: Option<Primitive<Uri>>,
        identifier: Option<Box<Identifier>>,
        display: Option<Primitive<FhirString>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> FhirCoreResult<Self> {
        Self::validate_ref2(&reference, &identifier, &display, &extension)?;
        Ok(Self {
            id,
            extension,
            reference,
            r#type,
            identifier,
            display,
        })
    }

    /// Validates the `ref-2` invariant: at least one of `reference`, `identifier`,
    /// `display`, or `extension` must be present (`type` does not count).
    fn validate_ref2(
        reference: &Option<Primitive<FhirString>>,
        identifier: &Option<Box<Identifier>>,
        display: &Option<Primitive<FhirString>>,
        extension: &[Extension],
    ) -> Result<(), ConstraintError> {
        if reference.is_none() && identifier.is_none() && display.is_none() && extension.is_empty()
        {
            return Err(ConstraintError::InvariantViolated {
                key: "ref-2",
                description: "At least one of reference, identifier and display SHALL be present (unless an extension is provided)".to_owned(),
            });
        }
        Ok(())
    }

    /// Creates a new `Reference` without validating the `ref-2` invariant.
    ///
    /// # Warning
    /// The caller is responsible for ensuring at least one field satisfies `ref-2`.
    #[inline]
    pub fn new_unchecked(
        reference: Option<Primitive<FhirString>>,
        r#type: Option<Primitive<Uri>>,
        identifier: Option<Box<Identifier>>,
        display: Option<Primitive<FhirString>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> Self {
        Self {
            id,
            extension,
            reference,
            r#type,
            identifier,
            display,
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

    /// Returns the literal reference string, if present.
    #[inline]
    pub fn reference(&self) -> Option<&Primitive<FhirString>> {
        self.reference.as_ref()
    }

    /// Returns the expected target resource type, if present.
    #[inline]
    pub fn r#type(&self) -> Option<&Primitive<Uri>> {
        self.r#type.as_ref()
    }

    /// Returns the logical reference identifier, if present.
    #[inline]
    pub fn identifier(&self) -> Option<&Identifier> {
        self.identifier.as_deref()
    }

    /// Returns the plain-text display, if present.
    #[inline]
    pub fn display(&self) -> Option<&Primitive<FhirString>> {
        self.display.as_ref()
    }
}
