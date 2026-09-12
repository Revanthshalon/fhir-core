//! The FHIR `MoneyQuantity` complex data type (a constrained `Quantity` profile).
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Quantity)
//! Structurally identical to [`Quantity`](crate::datatypes::complex::quantity::Quantity)
//! — the profile constrains `code`/`system` to a currency unit (ISO 4217). No
//! additional fields beyond `Quantity`'s. Note: the spec favors plain
//! [`Money`](crate::datatypes::complex::money::Money) for new content; this profile is
//! kept mainly for backward compatibility.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::primitive::Primitive;
use crate::types::{Code, Decimal, FhirString, Uri};

/// The FHIR `MoneyQuantity` complex data type: a `Quantity` constrained to a currency
/// unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoneyQuantity {
    value: Option<Primitive<Decimal>>,
    comparator: Option<Primitive<Code>>,
    unit: Option<Primitive<FhirString>>,
    system: Option<Primitive<Uri>>,
    code: Option<Primitive<Code>>,
}
