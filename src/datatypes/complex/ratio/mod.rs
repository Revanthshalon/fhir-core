//! The FHIR `Ratio` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Ratio)
//! - Fields: `numerator` (0..1,
//!   [`Quantity`](crate::datatypes::complex::quantity::Quantity)), `denominator`
//!   (0..1, [`SimpleQuantity`](crate::datatypes::complex::simple_quantity::SimpleQuantity)),
//!   plus `id`/`extension`.
//! - No named invariants found; re-check datatypes-definitions.html before assuming
//!   none exists.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `Quantity` and
//! `SimpleQuantity`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::quantity::Quantity;
use crate::datatypes::complex::simple_quantity::SimpleQuantity;

/// The FHIR `Ratio` complex data type: a numerator quantity over a denominator
/// quantity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ratio {
    numerator: Option<Quantity>,
    denominator: Option<SimpleQuantity>,
}
