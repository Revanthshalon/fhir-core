//! The FHIR `Money` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Money)
//! - Fields: `value` (0..1, `decimal`), `currency` (0..1, `code` — ISO 4217 currency
//!   code), plus `id`/`extension` (every type inherits these from `Element`).
//! - No named invariants found.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::primitive::Primitive;
use crate::types::{Code, Decimal, FhirString};

/// The FHIR `Money` complex data type: a decimal amount in a named currency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    value: Option<Primitive<Decimal>>,
    currency: Option<Primitive<Code>>,
}
