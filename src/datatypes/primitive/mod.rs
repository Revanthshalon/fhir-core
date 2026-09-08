//! The FHIR `Primitive<T>` element wrapper.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/types.html#Element, hl7.org/fhir/R5/json.html)
//! - Every FHIR primitive-valued property is, structurally, an `Element`: a value plus
//!   optional `id` and `extension`. In JSON this is represented as up to two sibling
//!   keys — a bare `propertyName` for the value, and a `_propertyName` companion object
//!   for `id`/`extension` — present only as needed (see `Extension`'s hand-rolled
//!   serde for exactly how the pair is split/merged; `Primitive<T>` itself intentionally
//!   has no `Serialize`/`Deserialize`, because it isn't representable as a single JSON
//!   value in general).
//! - Invariant `ele-1` (FHIRPath: `hasValue() or (children().count() > id.count())`):
//!   "All FHIR elements must have a @value or children." `id` counts as a child in the
//!   FHIRPath `children()` count, so `id.count()` cancels it out of the comparison —
//!   **`id` alone, with no value and no extension, does not satisfy `ele-1`.** Only
//!   `value` present, or `extension` non-empty, counts.
//! - `Element.id` (e.g. `Extension.id`) is a plain ordinary property, never itself
//!   wrapped in a `_id` companion — see `Extension`'s module docs.
//!
//! # Usage
//! Use [`Primitive::new`] for the general case (validates `ele-1`), or
//! [`Primitive::from_value`] for the common "just a value, no metadata" case, which is
//! infallible since a present value always satisfies `ele-1`.

use serde::{Deserialize, Serialize};

use crate::datatypes::complex::Extension;
use crate::errors::{FhirCoreResult, constraints::ConstraintError};
use crate::types::FhirString;

#[cfg(test)]
mod test;

/// The `_propertyName` companion JSON object (`{"id": ..., "extension": [...]}`) that a
/// container splits a [`Primitive<T>`] into alongside its bare `propertyName` value key.
///
/// Not public API: an implementation detail of how hand-rolled container `Serialize`/
/// `Deserialize` impls (e.g. `Extension`'s) represent a `Primitive<T>` across its two
/// possible JSON keys. See the module docs for why `Primitive<T>` has no `Serialize`/
/// `Deserialize` of its own.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct PrimitiveCompanion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<FhirString>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) extension: Vec<Extension>,
}

/// Serializes `primitive` as its `key`/`{underscore_key}` JSON pair into `map`, emitting
/// only whichever of the two is actually populated. Used by every hand-rolled container
/// `Serialize` impl (currently just `Extension`) so the split logic lives in one place.
pub(crate) fn serialize_primitive_entry<M, T>(
    map: &mut M,
    key: &str,
    underscore_key: &str,
    primitive: &Primitive<T>,
) -> Result<(), M::Error>
where
    M: serde::ser::SerializeMap,
    T: Serialize,
{
    if let Some(v) = primitive.value() {
        map.serialize_entry(key, v)?;
    }
    if primitive.id().is_some() || !primitive.extensions().is_empty() {
        let companion = PrimitiveCompanion {
            id: primitive.id().cloned(),
            extension: primitive.extensions().to_vec(),
        };
        map.serialize_entry(underscore_key, &companion)?;
    }
    Ok(())
}

/// Merges a candidate bare value and a candidate `_`-prefixed companion (either or both
/// may be absent) back into one [`Primitive<T>`], validating `ele-1`. Used by every
/// hand-rolled container `Deserialize` impl once it has scanned both possible JSON keys.
pub(crate) fn merge_primitive_entry<T, E>(
    value: Option<T>,
    companion: Option<PrimitiveCompanion>,
) -> Result<Primitive<T>, E>
where
    E: serde::de::Error,
{
    let companion = companion.unwrap_or_default();
    Primitive::new(value, companion.id, companion.extension).map_err(E::custom)
}

/// Wraps a FHIR primitive value together with its optional `id` and `extension`.
///
/// # Invariants
/// Any instance of `Primitive<T>` constructed via [`Primitive::new`] or
/// [`Primitive::from_value`] is guaranteed to satisfy `ele-1`: `value` is `Some`, or
/// `extension` is non-empty (or both).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Primitive<T> {
    value: Option<T>,
    id: Option<FhirString>,
    extension: Vec<Extension>,
}

impl<T> Primitive<T> {
    /// Creates a new `Primitive`, validating the `ele-1` invariant.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Constraint`](crate::errors::FhirCoreError::Constraint)
    /// containing [`ConstraintError::InvariantViolated`] if `value` is `None` and
    /// `extension` is empty (`id` alone does not satisfy `ele-1`).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::datatypes::primitive::Primitive;
    /// use fhir_core::types::Boolean;
    ///
    /// let p = Primitive::new(Some(Boolean::new(true)), None, Vec::new());
    /// assert!(p.is_ok());
    ///
    /// let empty: Result<Primitive<Boolean>, _> = Primitive::new(None, None, Vec::new());
    /// assert!(empty.is_err());
    /// ```
    pub fn new(
        value: Option<T>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> FhirCoreResult<Self> {
        Self::validate_ele1(&value, &extension)?;
        Ok(Self {
            value,
            id,
            extension,
        })
    }

    /// Validates the `ele-1` invariant against candidate `value`/`extension` fields.
    fn validate_ele1(value: &Option<T>, extension: &[Extension]) -> Result<(), ConstraintError> {
        if value.is_none() && extension.is_empty() {
            return Err(ConstraintError::InvariantViolated {
                key: "ele-1",
                description: "All FHIR elements must have a @value or children".to_owned(),
            });
        }
        Ok(())
    }

    /// Creates a new `Primitive` wrapping a bare value, with no `id` or `extension`.
    ///
    /// Infallible: a present value always satisfies `ele-1`.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::datatypes::primitive::Primitive;
    /// use fhir_core::types::Boolean;
    ///
    /// let p = Primitive::from_value(Boolean::new(true));
    /// assert_eq!(p.value(), Some(&Boolean::new(true)));
    /// ```
    #[inline]
    pub fn from_value(value: T) -> Self {
        Self {
            value: Some(value),
            id: None,
            extension: Vec::new(),
        }
    }

    /// Creates a new `Primitive` without validating the `ele-1` invariant.
    ///
    /// # Warning
    /// The caller is responsible for ensuring `value` and `extension` satisfy `ele-1`.
    #[inline]
    pub fn new_unchecked(
        value: Option<T>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> Self {
        Self {
            value,
            id,
            extension,
        }
    }

    /// Returns the wrapped value, if present.
    #[inline]
    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Returns the element-level identifier, if present.
    #[inline]
    pub fn id(&self) -> Option<&FhirString> {
        self.id.as_ref()
    }

    /// Returns the slice of extensions attached to this element.
    #[inline]
    pub fn extensions(&self) -> &[Extension] {
        &self.extension
    }

    /// Consumes the `Primitive`, returning the wrapped value, if present.
    #[inline]
    pub fn into_value(self) -> Option<T> {
        self.value
    }
}
