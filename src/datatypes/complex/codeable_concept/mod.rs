//! The FHIR `CodeableConcept` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#CodeableConcept)
//! - Fields: `coding` (0..*, [`Coding`](crate::datatypes::complex::coding::Coding) — a
//!   reference to a code defined by a terminology system), `text` (0..1, `string` — the
//!   human language representation as seen/selected/uttered by the user).
//! - No named invariants.
//! - `text` is primitive-valued, so it's a
//!   [`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern; `coding` is a plain `Vec<Coding>` (a complex type doesn't get the
//!   companion treatment, only primitives do — see `Extension`'s module docs).
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap step 4 (`CodeableConcept`) before implementing:
//! re-verify this doc against the live spec, then add the constructor, accessors, and
//! serde. Depends on `Coding` (step 2) being implemented first.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::coding::Coding;
use crate::datatypes::primitive::Primitive;
use crate::types::FhirString;

/// The FHIR `CodeableConcept` complex data type: a concept expressed as one or more
/// codes plus an optional free-text representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeableConcept {
    coding: Vec<Coding>,
    text: Option<Primitive<FhirString>>,
}
