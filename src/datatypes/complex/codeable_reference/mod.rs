//! The FHIR `CodeableReference` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/references.html#CodeableReference)
//! - Fields: `concept` (0..1,
//!   [`CodeableConcept`](crate::datatypes::complex::codeable_concept::CodeableConcept)
//!   — the general class of the referenced thing), `reference` (0..1,
//!   [`Reference`](crate::datatypes::complex::reference::Reference) — the specific
//!   instance), plus `id`/`extension`.
//! - No named invariant on `CodeableReference` itself; if `reference` is populated it
//!   is still subject to `Reference`'s own `ref-2` (already enforced by
//!   `Reference::new`, since `reference` here holds a real `Reference` value).
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `CodeableConcept` and
//! `Reference`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::codeable_concept::CodeableConcept;
use crate::datatypes::complex::reference::Reference;
use crate::types::FhirString;

/// The FHIR `CodeableReference` complex data type: either (or both) a general
/// classification concept and a specific resource reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeableReference {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    concept: Option<CodeableConcept>,
    reference: Option<Reference>,
}
