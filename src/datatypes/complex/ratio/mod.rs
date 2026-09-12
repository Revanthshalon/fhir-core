//! The FHIR `Ratio` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Ratio)
//! - Fields: `numerator` (0..1,
//!   [`Quantity`](crate::datatypes::complex::quantity::Quantity)), `denominator`
//!   (0..1, [`SimpleQuantity`](crate::datatypes::complex::simple_quantity::SimpleQuantity)),
//!   plus `id`/`extension`.
//! - Invariant `rat-1` (**error** severity, confirmed via datatypes-definitions.html,
//!   not yet enforced in this stub): "Numerator and denominator SHALL both be present,
//!   or both are absent. If both are absent, there SHALL be some extension present"
//!   (FHIRPath: `(numerator.exists() and denominator.exists()) or (numerator.empty()
//!   and denominator.empty() and extension.exists())`) — this one is a plain presence
//!   check like `Quantity`'s `qty-3`, fully implementable without boundary-expansion
//!   support.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `Quantity` and
//! `SimpleQuantity`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::quantity::Quantity;
use crate::datatypes::complex::simple_quantity::SimpleQuantity;
use crate::types::FhirString;

/// The FHIR `Ratio` complex data type: a numerator quantity over a denominator
/// quantity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ratio {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    numerator: Option<Quantity>,
    denominator: Option<SimpleQuantity>,
}
