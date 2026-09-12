//! The FHIR `ContactPoint` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#ContactPoint)
//! - Fields: `system` (0..1, `code`), `value` (0..1, `string`), `use` (0..1, `code`),
//!   `rank` (0..1, `positiveInt`), `period` (0..1,
//!   [`Period`](crate::datatypes::complex::period::Period)), plus `id`/`extension`.
//! - No named invariants found.
//! - `use` is a Rust keyword — kept as the raw identifier `r#use`, matching this
//!   crate's `Identifier`/`TypeError` convention.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `Period`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::period::Period;
use crate::datatypes::primitive::Primitive;
use crate::types::{Code, FhirString, PositiveInt};

/// The FHIR `ContactPoint` complex data type: a technology-mediated contact detail
/// (phone, fax, email, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactPoint {
    system: Option<Primitive<Code>>,
    value: Option<Primitive<FhirString>>,
    r#use: Option<Primitive<Code>>,
    rank: Option<Primitive<PositiveInt>>,
    period: Option<Period>,
}
