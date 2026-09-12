//! The FHIR `Expression` metadata type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/metadatatypes.html#Expression)
//! - Fields: `description` (0..1, `string`), `name` (0..1, `code` — matches
//!   `[A-Za-z][A-Za-z0-9_]{0,63}` when present), `language` (0..1, `code` — media type
//!   e.g. `text/cql`, `text/fhirpath`), `expression` (0..1, `string`), `reference`
//!   (0..1, `uri`), plus `id`/`extension`.
//! - Invariant `exp-1` (**error** severity, not yet enforced in this stub): "An
//!   expression or a reference must be provided" (at least one of `expression`,
//!   `reference` present).
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec
//! (cross-checked here against a non-pinned build, not the frozen R5 5.0.0 page — a
//! harder requirement than usual to re-verify before implementing), then add the
//! constructor (enforcing `exp-1`), accessors, and serde.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::primitive::Primitive;
use crate::types::{Code, FhirString, Uri};

/// The FHIR `Expression` metadata type: a computable, language-tagged expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    description: Option<Primitive<FhirString>>,
    name: Option<Primitive<Code>>,
    language: Option<Primitive<Code>>,
    expression: Option<Primitive<FhirString>>,
    reference: Option<Primitive<Uri>>,
}
