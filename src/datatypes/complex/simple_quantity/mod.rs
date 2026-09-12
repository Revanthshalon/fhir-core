//! The FHIR `SimpleQuantity` complex data type (a constrained `Quantity` profile).
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Quantity)
//! Structurally identical to [`Quantity`](crate::datatypes::complex::quantity::Quantity)
//! (`value`, `comparator`, `unit`, `system`, `code`, plus `id`/`extension`) — the
//! profile just adds constraint `sqty-1`: `comparator` SHALL NOT be present (a simple
//! quantity is always an exact value, never open-ended).
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor (enforcing `sqty-1`), accessors, and serde.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::primitive::Primitive;
use crate::types::{Code, Decimal, FhirString, Uri};

/// The FHIR `SimpleQuantity` complex data type: a `Quantity` constrained to never carry
/// a `comparator` (`sqty-1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleQuantity {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    value: Option<Primitive<Decimal>>,
    unit: Option<Primitive<FhirString>>,
    system: Option<Primitive<Uri>>,
    code: Option<Primitive<Code>>,
}
