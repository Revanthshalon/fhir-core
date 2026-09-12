//! The FHIR `Distance` complex data type (a constrained `Quantity` profile).
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Quantity)
//! Structurally identical to [`Quantity`](crate::datatypes::complex::quantity::Quantity)
//! — the profile constrains `code`/`system` to a UCUM length unit when present. No
//! additional fields beyond `Quantity`'s.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::primitive::Primitive;
use crate::types::{Code, Decimal, FhirString, Uri};

/// The FHIR `Distance` complex data type: a `Quantity` constrained to a length unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Distance {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    value: Option<Primitive<Decimal>>,
    comparator: Option<Primitive<Code>>,
    unit: Option<Primitive<FhirString>>,
    system: Option<Primitive<Uri>>,
    code: Option<Primitive<Code>>,
}
