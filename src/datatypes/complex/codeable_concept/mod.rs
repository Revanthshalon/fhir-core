//! The FHIR `CodeableConcept` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#CodeableConcept,
//! hl7.org/fhir/R5/datatypes-definitions.html#CodeableConcept.text)
//! - Fields: `coding` (0..*, [`Coding`] — a reference to a code defined by a
//!   terminology system), `text` (0..1, `string` — the human language representation
//!   as seen/selected/uttered by the user).
//! - No named invariants for `CodeableConcept` itself (double-checked against
//!   datatypes-definitions.html, not just datatypes.html, per this crate's rule — see
//!   the `Period`/`Coding` roadmap corrections for why that distinction matters). The
//!   only invariant mentioning this type is `Coding`'s own `cod-1`, already enforced
//!   (as a documented non-enforcement) on `Coding` itself.
//! - Like every FHIR type, `CodeableConcept` inherits `id` (0..1, `string`) and
//!   `extension` (0..*, `Extension`) from `Element`, and is therefore itself subject to
//!   `ele-1` ("must have a value or children") — with no primitive `value` of its own,
//!   at least one of `coding` (non-empty), `text`, or `extension` must be present.
//! - `text` is primitive-valued, so it's a
//!   [`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern; `coding` is a plain `Vec<Coding>` (a complex type doesn't get the
//!   companion treatment, only primitives do — see `Extension`'s module docs).
//!   `CodeableConcept.id` is a plain ordinary property, never itself
//!   companion-wrapped, matching `Extension.id`/`Period.id`/`Coding.id`.
//!
//! # Usage
//! Use [`CodeableConcept::new`] to construct a validated instance (checks `ele-1`), or
//! [`CodeableConcept::new_unchecked`] when the fields are already known to satisfy it.

#[cfg(feature = "serde")]
use serde::de::{Error as DeError, MapAccess, Visitor};
#[cfg(feature = "serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::coding::Coding;
use crate::datatypes::primitive::Primitive;
#[cfg(feature = "serde")]
use crate::datatypes::primitive::{
    PrimitiveCompanion, merge_primitive_entry, serialize_primitive_entry,
};
use crate::errors::{FhirCoreResult, constraints::ConstraintError};
use crate::types::FhirString;

#[cfg(test)]
mod test;

/// The FHIR `CodeableConcept` complex data type: a concept expressed as one or more
/// codes plus an optional free-text representation.
///
/// # Invariants
/// Any instance of `CodeableConcept` constructed via [`CodeableConcept::new`] is
/// guaranteed to satisfy `ele-1`: at least one of `coding` (non-empty), `text`, or
/// `extension` is set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeableConcept {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    coding: Vec<Coding>,
    text: Option<Primitive<FhirString>>,
}

#[cfg(feature = "serde")]
impl Serialize for CodeableConcept {
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
        if !self.coding.is_empty() {
            map.serialize_entry("coding", &self.coding)?;
        }
        if let Some(text) = &self.text {
            serialize_primitive_entry(&mut map, "text", "_text", text)?;
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
impl<'de> Deserialize<'de> for CodeableConcept {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CodeableConceptVisitor;

        impl<'de> Visitor<'de> for CodeableConceptVisitor {
            type Value = CodeableConcept;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FHIR CodeableConcept JSON object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<CodeableConcept, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut extension = Vec::new();
                let mut coding = Vec::new();
                let mut text: Slot<FhirString> = Slot::default();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => id = Some(map.next_value()?),
                        "extension" => extension = map.next_value()?,
                        "coding" => coding = map.next_value()?,
                        "text" => text.value = Some(map.next_value()?),
                        "_text" => text.companion = Some(map.next_value()?),
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let text = text
                    .is_present()
                    .then(|| merge_primitive_entry(text.value, text.companion))
                    .transpose()?;

                CodeableConcept::new(coding, text, id, extension).map_err(DeError::custom)
            }
        }

        deserializer.deserialize_map(CodeableConceptVisitor)
    }
}

impl CodeableConcept {
    /// Creates a new `CodeableConcept`, validating the `ele-1` invariant.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Constraint`](crate::errors::FhirCoreError::Constraint)
    /// containing [`ConstraintError::InvariantViolated`] if `coding`, `text`, and
    /// `extension` are all absent/empty.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::datatypes::complex::codeable_concept::CodeableConcept;
    /// use fhir_core::datatypes::primitive::Primitive;
    /// use fhir_core::types::FhirString;
    ///
    /// let text = Primitive::from_value(FhirString::new("Active").unwrap());
    /// let cc = CodeableConcept::new(Vec::new(), Some(text), None, Vec::new());
    /// assert!(cc.is_ok());
    ///
    /// let empty = CodeableConcept::new(Vec::new(), None, None, Vec::new());
    /// assert!(empty.is_err());
    /// ```
    pub fn new(
        coding: Vec<Coding>,
        text: Option<Primitive<FhirString>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> FhirCoreResult<Self> {
        Self::validate_ele1(&coding, &text, &extension)?;
        Ok(Self {
            id,
            extension,
            coding,
            text,
        })
    }

    /// Validates the `ele-1` invariant against candidate `coding`/`text`/`extension` fields.
    fn validate_ele1(
        coding: &[Coding],
        text: &Option<Primitive<FhirString>>,
        extension: &[Extension],
    ) -> Result<(), ConstraintError> {
        if coding.is_empty() && text.is_none() && extension.is_empty() {
            return Err(ConstraintError::InvariantViolated {
                key: "ele-1",
                description: "All FHIR elements must have a @value or children".to_owned(),
            });
        }
        Ok(())
    }

    /// Creates a new `CodeableConcept` without validating the `ele-1` invariant.
    ///
    /// # Warning
    /// The caller is responsible for ensuring `coding`, `text`, and `extension` satisfy
    /// `ele-1`.
    #[inline]
    pub fn new_unchecked(
        coding: Vec<Coding>,
        text: Option<Primitive<FhirString>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> Self {
        Self {
            id,
            extension,
            coding,
            text,
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

    /// Returns the slice of codings.
    #[inline]
    pub fn coding(&self) -> &[Coding] {
        &self.coding
    }

    /// Returns the free-text representation, if present.
    #[inline]
    pub fn text(&self) -> Option<&Primitive<FhirString>> {
        self.text.as_ref()
    }
}
