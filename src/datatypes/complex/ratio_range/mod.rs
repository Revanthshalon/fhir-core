//! The FHIR `RatioRange` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#RatioRange)
//! - Fields: `lowNumerator` (0..1,
//!   [`SimpleQuantity`](crate::datatypes::complex::simple_quantity::SimpleQuantity)),
//!   `highNumerator` (0..1, `SimpleQuantity`), `denominator` (0..1, `SimpleQuantity`),
//!   plus `id`/`extension`.
//! - No named invariants found; re-check datatypes-definitions.html before assuming
//!   none exists.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `SimpleQuantity`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::simple_quantity::SimpleQuantity;

/// The FHIR `RatioRange` complex data type: a ratio expressed as a bounded numerator
/// range over a fixed denominator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatioRange {
    low_numerator: Option<SimpleQuantity>,
    high_numerator: Option<SimpleQuantity>,
    denominator: Option<SimpleQuantity>,
}
