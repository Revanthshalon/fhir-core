//! The FHIR `Period` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Period,
//! hl7.org/fhir/R5/datatypes-definitions.html#Period.end)
//! - Fields: `start` (0..1, `dateTime`, inclusive lower boundary), `end` (0..1,
//!   `dateTime` — absent means "no end known or planned", not "ongoing forever"; the
//!   end value is itself included in the period, e.g. an `end` of `2012-02-03` includes
//!   `2012-02-03T10:00:00`).
//! - Invariant `per-1` (FHIRPath: `start.hasValue().not() or end.hasValue().not() or
//!   (start.lowBoundary() <= end.highBoundary())`): if both are present, `start` must
//!   be less than or equal to `end`, compared via FHIRPath's boundary functions (which
//!   expand a partial-precision value like `"2020"` to its earliest/latest possible
//!   instant before comparing) — not a plain string/lexicographic comparison.
//! - Like every FHIR type, `Period` inherits `id` (0..1, `string`) and `extension`
//!   (0..*, `Extension`) from `Element`, and is therefore itself subject to `ele-1`
//!   ("must have a value or children") — with no primitive `value` of its own, that
//!   means at least one of `start`, `end`, or `extension` must be present.
//! - `start`/`end` are primitive-valued, so both are
//!   [`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern (see `Extension`'s module docs). `Period.id` is a plain ordinary property,
//!   never itself companion-wrapped, matching `Extension.id`.
//!
//! # Known gap: `per-1` is not validated
//! Correctly evaluating `per-1` requires FHIRPath's `lowBoundary()`/`highBoundary()`
//! semantics: expanding a partial-precision `dateTime` (e.g. `"2020"`, `"2020-06"`) to
//! its earliest/latest possible instant, and comparing across differing timezone
//! offsets. `DateTime` (`src/types/date_time/mod.rs`) has no such boundary-expansion or
//! true chronological comparison today — its derived `Ord` is plain string ordering,
//! which is only chronologically correct when both values share the same precision and
//! timezone offset. Validating `per-1` with that would silently accept or reject the
//! wrong periods for mixed-precision/mixed-timezone input, which is worse than not
//! checking. `Period::new` therefore validates only `ele-1`. Add `per-1` once `DateTime`
//! gains real boundary/comparison support — same "don't fake a partial check" call as
//! `Reference`'s `ref-1` gap.
//!
//! # Usage
//! Use [`Period::new`] to construct a validated instance (checks `ele-1`), or
//! [`Period::new_unchecked`] when the fields are already known to satisfy it.

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
use crate::types::{DateTime, FhirString};

#[cfg(test)]
mod test;

/// The FHIR `Period` complex data type: a time range with an inclusive start and an
/// optional, possibly-unknown end.
///
/// # Invariants
/// Any instance of `Period` constructed via [`Period::new`] is guaranteed to satisfy
/// `ele-1`: at least one of `start`, `end`, or `extension` is set. See the module docs
/// for why `per-1` (start <= end) is not validated here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Period {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    start: Option<Primitive<DateTime>>,
    end: Option<Primitive<DateTime>>,
}

#[cfg(feature = "serde")]
impl Serialize for Period {
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
        if let Some(start) = &self.start {
            serialize_primitive_entry(&mut map, "start", "_start", start)?;
        }
        if let Some(end) = &self.end {
            serialize_primitive_entry(&mut map, "end", "_end", end)?;
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

// Manual impl: `#[derive(Default)]` would require `T: Default`, which `DateTime`
// doesn't implement (and shouldn't need to).
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
impl<'de> Deserialize<'de> for Period {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PeriodVisitor;

        impl<'de> Visitor<'de> for PeriodVisitor {
            type Value = Period;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FHIR Period JSON object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Period, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut extension = Vec::new();
                let mut start: Slot<DateTime> = Slot::default();
                let mut end: Slot<DateTime> = Slot::default();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => id = Some(map.next_value()?),
                        "extension" => extension = map.next_value()?,
                        "start" => start.value = Some(map.next_value()?),
                        "_start" => start.companion = Some(map.next_value()?),
                        "end" => end.value = Some(map.next_value()?),
                        "_end" => end.companion = Some(map.next_value()?),
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let start = start
                    .is_present()
                    .then(|| merge_primitive_entry(start.value, start.companion))
                    .transpose()?;
                let end = end
                    .is_present()
                    .then(|| merge_primitive_entry(end.value, end.companion))
                    .transpose()?;

                Period::new(start, end, id, extension).map_err(DeError::custom)
            }
        }

        deserializer.deserialize_map(PeriodVisitor)
    }
}

impl Period {
    /// Creates a new `Period`, validating the `ele-1` invariant.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Constraint`](crate::errors::FhirCoreError::Constraint)
    /// containing [`ConstraintError::InvariantViolated`] if `start`, `end`, and
    /// `extension` are all absent/empty.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::datatypes::complex::period::Period;
    /// use fhir_core::datatypes::primitive::Primitive;
    /// use fhir_core::types::DateTime;
    ///
    /// let start = Primitive::from_value(DateTime::new("2024-01-01T00:00:00Z").unwrap());
    /// let period = Period::new(Some(start), None, None, Vec::new());
    /// assert!(period.is_ok());
    ///
    /// let empty = Period::new(None, None, None, Vec::new());
    /// assert!(empty.is_err());
    /// ```
    pub fn new(
        start: Option<Primitive<DateTime>>,
        end: Option<Primitive<DateTime>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> FhirCoreResult<Self> {
        Self::validate_ele1(&start, &end, &extension)?;
        Ok(Self {
            id,
            extension,
            start,
            end,
        })
    }

    /// Validates the `ele-1` invariant against candidate `start`/`end`/`extension` fields.
    fn validate_ele1(
        start: &Option<Primitive<DateTime>>,
        end: &Option<Primitive<DateTime>>,
        extension: &[Extension],
    ) -> Result<(), ConstraintError> {
        if start.is_none() && end.is_none() && extension.is_empty() {
            return Err(ConstraintError::InvariantViolated {
                key: "ele-1",
                description: "All FHIR elements must have a @value or children".to_owned(),
            });
        }
        Ok(())
    }

    /// Creates a new `Period` without validating the `ele-1` invariant.
    ///
    /// # Warning
    /// The caller is responsible for ensuring `start`, `end`, and `extension` satisfy
    /// `ele-1`.
    #[inline]
    pub fn new_unchecked(
        start: Option<Primitive<DateTime>>,
        end: Option<Primitive<DateTime>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> Self {
        Self {
            id,
            extension,
            start,
            end,
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

    /// Returns the inclusive start of the period, if present.
    #[inline]
    pub fn start(&self) -> Option<&Primitive<DateTime>> {
        self.start.as_ref()
    }

    /// Returns the end of the period, if present.
    #[inline]
    pub fn end(&self) -> Option<&Primitive<DateTime>> {
        self.end.as_ref()
    }
}
