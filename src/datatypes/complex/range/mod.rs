//! The FHIR `Range` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Range)
//! - Fields: `low` (0..1,
//!   [`SimpleQuantity`](crate::datatypes::complex::simple_quantity::SimpleQuantity) —
//!   inclusive lower boundary), `high` (0..1, `SimpleQuantity` — inclusive upper
//!   boundary), plus `id`/`extension`.
//! - No named invariants found on the summary page; re-check
//!   datatypes-definitions.html for a `low` <= `high` style rule before assuming none
//!   exists.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `SimpleQuantity`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::simple_quantity::SimpleQuantity;

/// The FHIR `Range` complex data type: an inclusive lower/upper quantity bound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range {
    low: Option<SimpleQuantity>,
    high: Option<SimpleQuantity>,
}
