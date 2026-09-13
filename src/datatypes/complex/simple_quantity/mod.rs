//! The FHIR `SimpleQuantity` complex data type (a constrained `Quantity` profile).
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Quantity,
//! <http://hl7.org/fhir/StructureDefinition/SimpleQuantity>)
//! Structurally identical to [`Quantity`](crate::datatypes::complex::quantity::Quantity)
//! minus `comparator` — `value`, `unit`, `system`, `code`, plus `id`/`extension`.
//!
//! Confirmed invariant (source: the `SimpleQuantity` profile StructureDefinition JSON,
//! error severity):
//! - `sqty-1`: `comparator.empty()` — "The comparator is not used on a
//!   SimpleQuantity." This crate enforces it structurally: `comparator` is not a field
//!   on this struct at all, so `sqty-1` cannot be violated by construction.
//! - `qty-3` (inherited from `Quantity`): `code.empty() or system.exists()` — enforced
//!   here.
//! - `ele-1` (inherited from `Element`) — enforced here.
//!
//! # Usage
//! Use [`SimpleQuantity::new`] to construct a validated instance, or
//! [`SimpleQuantity::new_unchecked`] when the fields are already known to satisfy the
//! invariants.

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

/// The FHIR `SimpleQuantity` complex data type: a `Quantity` that never carries a
/// `comparator` (`sqty-1`), enforced structurally by omitting the field.
///
/// # Invariants
/// Any instance constructed via [`SimpleQuantity::new`] is guaranteed to satisfy
/// `ele-1` and `qty-3`; `sqty-1` holds unconditionally since there is no `comparator`
/// field to violate it with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleQuantity {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    value: Option<Primitive<Decimal>>,
    unit: Option<Primitive<FhirString>>,
    system: Option<Primitive<Uri>>,
    code: Option<Primitive<Code>>,
}

#[cfg(feature = "serde")]
impl Serialize for SimpleQuantity {
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
impl<'de> Deserialize<'de> for SimpleQuantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SimpleQuantityVisitor;

        impl<'de> Visitor<'de> for SimpleQuantityVisitor {
            type Value = SimpleQuantity;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FHIR SimpleQuantity JSON object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<SimpleQuantity, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut extension = Vec::new();
                let mut value: Slot<Decimal> = Slot::default();
                let mut unit: Slot<FhirString> = Slot::default();
                let mut system: Slot<Uri> = Slot::default();
                let mut code: Slot<Code> = Slot::default();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => id = Some(map.next_value()?),
                        "extension" => extension = map.next_value()?,
                        "value" => value.value = Some(map.next_value()?),
                        "_value" => value.companion = Some(map.next_value()?),
                        "unit" => unit.value = Some(map.next_value()?),
                        "_unit" => unit.companion = Some(map.next_value()?),
                        "system" => system.value = Some(map.next_value()?),
                        "_system" => system.companion = Some(map.next_value()?),
                        "code" => code.value = Some(map.next_value()?),
                        "_code" => code.companion = Some(map.next_value()?),
                        // `comparator` is deliberately not a field on this struct
                        // (sqty-1) — a JSON payload carrying one is simply ignored,
                        // consistent with the "unknown field" branch below, rather than
                        // treated as a deserialize error.
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let value = value
                    .is_present()
                    .then(|| merge_primitive_entry(value.value, value.companion))
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

                SimpleQuantity::new(value, unit, system, code, id, extension)
                    .map_err(DeError::custom)
            }
        }

        deserializer.deserialize_map(SimpleQuantityVisitor)
    }
}

impl SimpleQuantity {
    /// Creates a new `SimpleQuantity`, validating the `ele-1` and `qty-3` invariants
    /// (`sqty-1` holds unconditionally — see module docs).
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Constraint`](crate::errors::FhirCoreError::Constraint)
    /// containing [`ConstraintError::InvariantViolated`] if `ele-1` or `qty-3` is
    /// violated.
    pub fn new(
        value: Option<Primitive<Decimal>>,
        unit: Option<Primitive<FhirString>>,
        system: Option<Primitive<Uri>>,
        code: Option<Primitive<Code>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> FhirCoreResult<Self> {
        Self::validate_ele1(&value, &unit, &system, &code, &extension)?;
        Self::validate_qty3(&system, &code)?;
        Ok(Self {
            id,
            extension,
            value,
            unit,
            system,
            code,
        })
    }

    /// Validates the `ele-1` invariant against candidate fields.
    fn validate_ele1(
        value: &Option<Primitive<Decimal>>,
        unit: &Option<Primitive<FhirString>>,
        system: &Option<Primitive<Uri>>,
        code: &Option<Primitive<Code>>,
        extension: &[Extension],
    ) -> Result<(), ConstraintError> {
        if value.is_none()
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

    /// Creates a new `SimpleQuantity` without validating the `ele-1`/`qty-3`
    /// invariants.
    ///
    /// # Warning
    /// The caller is responsible for ensuring the fields satisfy both invariants.
    #[inline]
    pub fn new_unchecked(
        value: Option<Primitive<Decimal>>,
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
