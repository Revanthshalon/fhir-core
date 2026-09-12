//! The FHIR `Narrative` special-purpose data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/narrative.html#Narrative)
//! - Fields: `status` (1..1, `code` — generated | extensions | additional | empty),
//!   `div` (1..1, `xhtml`), plus `id`/`extension`.
//! - Two invariants on `div`, both **error** severity, neither enforced in this stub:
//!   the content SHALL be restricted to a limited HTML 4.0 subset (basic formatting,
//!   `<a>`, images, inline `style`), and SHALL have some non-whitespace content.
//! - **Blocked on a real gap, not just unimplemented**: `div`'s spec type is `xhtml`,
//!   which this crate does not implement as a primitive (`docs/BACKLOG.md` roadmap
//!   already tracks this — FHIR R5 defines 21 primitives, this crate has 20). `div` is
//!   represented here as `FhirString` purely as a placeholder so this struct compiles;
//!   it is **not** spec-accurate (`FhirString` doesn't enforce the HTML-subset
//!   restriction or "non-whitespace content" invariants above). Implementing
//!   `Narrative` for real requires implementing `xhtml` first.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet, and blocked
//! on the `xhtml` primitive gap above (not just "not picked up yet" like the other
//! stubs in this module). See `docs/BACKLOG.md` Roadmap.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::primitive::Primitive;
use crate::types::{Code, FhirString};

/// The FHIR `Narrative` special-purpose data type: a human-readable summary of a
/// resource, generated and/or authored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Narrative {
    status: Option<Primitive<Code>>,
    /// Placeholder for the real `xhtml`-typed `div` field — see module docs.
    div: Option<Primitive<FhirString>>,
}
