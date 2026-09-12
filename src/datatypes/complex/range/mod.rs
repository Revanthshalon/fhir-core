//! The FHIR `Range` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Range)
//! - Fields: `low` (0..1,
//!   [`SimpleQuantity`](crate::datatypes::complex::simple_quantity::SimpleQuantity) —
//!   inclusive lower boundary), `high` (0..1, `SimpleQuantity` — inclusive upper
//!   boundary), plus `id`/`extension`.
//! - Invariant `rng-2` (**error** severity, confirmed via datatypes-definitions.html,
//!   not yet enforced in this stub): "If present, `low` SHALL have a lower value than
//!   `high`" (FHIRPath: `low.value.empty() or high.value.empty() or
//!   low.lowBoundary().comparable(high.highBoundary()).not() or (low.lowBoundary() <=
//!   high.highBoundary())`) — the `lowBoundary()`/`highBoundary()` comparison has the
//!   same "no boundary-expansion support" blocker as `Period`'s `per-1`.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `SimpleQuantity`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::simple_quantity::SimpleQuantity;
use crate::types::FhirString;

/// The FHIR `Range` complex data type: an inclusive lower/upper quantity bound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    low: Option<SimpleQuantity>,
    high: Option<SimpleQuantity>,
}
