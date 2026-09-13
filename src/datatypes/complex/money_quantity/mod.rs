//! The FHIR `MoneyQuantity` complex data type (a constrained `Quantity` profile).
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Quantity,
//! <http://hl7.org/fhir/StructureDefinition/MoneyQuantity>)
//! Structurally identical to [`Quantity`](crate::datatypes::complex::quantity::Quantity):
//! `value` (0..1, `decimal`), `comparator` (0..1, `code`), `unit` (0..1, `string`),
//! `system` (0..1, `uri`), `code` (0..1, `code`), plus `id`/`extension`. Note: the spec
//! favors plain [`Money`](crate::datatypes::complex::money::Money) for new content;
//! this profile is kept mainly for backward compatibility.
//!
//! Confirmed invariants (source: the `MoneyQuantity` profile StructureDefinition JSON,
//! error severity throughout):
//! - `mtqy-1`: `(code.exists() or value.empty()) and (system.empty() or system =
//!   'urn:iso:std:iso:4217')` — "There SHALL be a code if there is a value and it SHALL
//!   be an expression of currency. If system is present, it SHALL be ISO 4217." Fully
//!   machine-checkable from `value`/`system`/`code` alone and enforced here.
//! - `qty-3` (inherited from `Quantity`): `code.empty() or system.exists()`.
//! - `ele-1` (inherited from `Element`).
//!
//! # Usage
//! Use [`MoneyQuantity::new`] to construct a validated instance, or
//! [`MoneyQuantity::new_unchecked`] when the fields are already known to satisfy the
//! invariants.
//!
//! # Ordering
//! `MoneyQuantity` implements `PartialOrd` (not `Ord`) via the same comparator-aware,
//! unit-aware magnitude comparison `Quantity` uses — see `quantity_magnitude`'s
//! module docs for exactly when comparisons return `None`.

#[cfg(feature = "serde")]
use serde::de::{Error as DeError, MapAccess, Visitor};
#[cfg(feature = "serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::quantity_magnitude::{QuantityMagnitude, quantity_partial_cmp};
use crate::datatypes::primitive::Primitive;
#[cfg(feature = "serde")]
use crate::datatypes::primitive::{
    PrimitiveCompanion, merge_primitive_entry, serialize_primitive_entry,
};
use crate::errors::{FhirCoreResult, constraints::ConstraintError};
use crate::types::{Code, Decimal, FhirString, Uri};

#[cfg(test)]
mod test;

/// The ISO 4217 currency unit-code system URI.
const ISO_4217: &str = "urn:iso:std:iso:4217";

/// The FHIR `MoneyQuantity` complex data type: a `Quantity` constrained to an ISO 4217
/// currency unit.
///
/// # Invariants
/// Any instance constructed via [`MoneyQuantity::new`] is guaranteed to satisfy
/// `ele-1`, `qty-3`, and `mtqy-1` (see module docs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoneyQuantity {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    value: Option<Primitive<Decimal>>,
    comparator: Option<Primitive<Code>>,
    unit: Option<Primitive<FhirString>>,
    system: Option<Primitive<Uri>>,
    code: Option<Primitive<Code>>,
}

#[cfg(feature = "serde")]
impl Serialize for MoneyQuantity {
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
impl<'de> Deserialize<'de> for MoneyQuantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MoneyQuantityVisitor;

        impl<'de> Visitor<'de> for MoneyQuantityVisitor {
            type Value = MoneyQuantity;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FHIR MoneyQuantity JSON object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<MoneyQuantity, A::Error>
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

                MoneyQuantity::new(value, comparator, unit, system, code, id, extension)
                    .map_err(DeError::custom)
            }
        }

        deserializer.deserialize_map(MoneyQuantityVisitor)
    }
}

impl MoneyQuantity {
    /// Creates a new `MoneyQuantity`, validating the `ele-1`, `qty-3`, and `mtqy-1`
    /// invariants.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Constraint`](crate::errors::FhirCoreError::Constraint)
    /// containing [`ConstraintError::InvariantViolated`] if any of the three invariants
    /// (see module docs) is violated.
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
        Self::validate_mtqy1(&value, &system, &code)?;
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

    /// Validates the `mtqy-1` invariant: a present `value` requires `code`, and a
    /// present `system` must be ISO 4217.
    fn validate_mtqy1(
        value: &Option<Primitive<Decimal>>,
        system: &Option<Primitive<Uri>>,
        code: &Option<Primitive<Code>>,
    ) -> Result<(), ConstraintError> {
        let description = "There SHALL be a code if there is a value and it SHALL be an \
            expression of currency. If system is present, it SHALL be ISO 4217."
            .to_owned();
        if value.is_some() && code.is_none() {
            return Err(ConstraintError::InvariantViolated {
                key: "mtqy-1",
                description,
            });
        }
        if system
            .as_ref()
            .and_then(Primitive::value)
            .is_some_and(|s| s.as_str() != ISO_4217)
        {
            return Err(ConstraintError::InvariantViolated {
                key: "mtqy-1",
                description,
            });
        }
        Ok(())
    }

    /// Creates a new `MoneyQuantity` without validating the `ele-1`/`qty-3`/`mtqy-1`
    /// invariants.
    ///
    /// # Warning
    /// The caller is responsible for ensuring the fields satisfy all three invariants.
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

    fn magnitude(&self) -> QuantityMagnitude<'_> {
        QuantityMagnitude {
            value: self.value.as_ref().and_then(Primitive::value),
            comparator: self.comparator.as_ref().and_then(Primitive::value),
            system: self.system.as_ref().and_then(Primitive::value),
            code: self.code.as_ref().and_then(Primitive::value),
        }
    }
}

impl PartialOrd for MoneyQuantity {
    /// Comparator-aware, unit-aware partial order. See the module docs' "Ordering"
    /// section for exactly when this returns `None`.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        match quantity_partial_cmp(self.magnitude(), other.magnitude()) {
            Some(std::cmp::Ordering::Equal) => None,
            ordering => ordering,
        }
    }
}
