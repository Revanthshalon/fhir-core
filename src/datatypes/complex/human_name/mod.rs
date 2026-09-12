//! The FHIR `HumanName` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#HumanName)
//! - Fields: `use` (0..1, `code`), `text` (0..1, `string`), `family` (0..1, `string`),
//!   `given` (0..*, `string`), `prefix` (0..*, `string`), `suffix` (0..*, `string`),
//!   `period` (0..1, [`Period`](crate::datatypes::complex::period::Period)), plus
//!   `id`/`extension`.
//! - No named invariants found.
//! - `use` is a Rust keyword — kept as the raw identifier `r#use`, matching this
//!   crate's `Identifier`/`TypeError` convention.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `Period`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::period::Period;
use crate::datatypes::primitive::Primitive;
use crate::types::{Code, FhirString};

/// The FHIR `HumanName` complex data type: a human name with the ability to identify
/// parts and usage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanName {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    r#use: Option<Primitive<Code>>,
    text: Option<Primitive<FhirString>>,
    family: Option<Primitive<FhirString>>,
    given: Vec<Primitive<FhirString>>,
    prefix: Vec<Primitive<FhirString>>,
    suffix: Vec<Primitive<FhirString>>,
    period: Option<Period>,
}
