//! The FHIR `ParameterDefinition` metadata type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/metadatatypes.html#ParameterDefinition)
//! - Fields: `name` (0..1, `code`), `use` (1..1, `code` — in | out), `min` (0..1,
//!   `integer`), `max` (0..1, `string`), `documentation` (0..1, `string`), `type`
//!   (1..1, `code`), `profile` (0..1, `canonical(StructureDefinition)`), plus
//!   `id`/`extension`.
//! - No named invariants found.
//! - `use` and `type` are Rust keywords — kept as raw identifiers `r#use`/`r#type`,
//!   matching this crate's `Identifier`/`TypeError` convention.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec
//! (cross-checked here against a non-pinned build, not the frozen R5 5.0.0 page — a
//! harder requirement than usual to re-verify before implementing), then add the
//! constructor, accessors, and serde.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::primitive::Primitive;
use crate::types::{Canonical, Code, FhirString, Integer};

/// The FHIR `ParameterDefinition` metadata type: an input or output parameter of a
/// module (e.g. an `OperationDefinition` or `PlanDefinition` action).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDefinition {
    name: Option<Primitive<Code>>,
    r#use: Option<Primitive<Code>>,
    min: Option<Primitive<Integer>>,
    max: Option<Primitive<FhirString>>,
    documentation: Option<Primitive<FhirString>>,
    r#type: Option<Primitive<Code>>,
    profile: Option<Primitive<Canonical>>,
}
