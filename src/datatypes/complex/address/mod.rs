//! The FHIR `Address` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Address)
//! - Fields: `use` (0..1, `code`), `type` (0..1, `code`), `text` (0..1, `string`),
//!   `line` (0..*, `string`), `city` (0..1, `string`), `district` (0..1, `string`),
//!   `state` (0..1, `string`), `postalCode` (0..1, `string`), `country` (0..1,
//!   `string`), `period` (0..1, [`Period`](crate::datatypes::complex::period::Period)),
//!   plus `id`/`extension`.
//! - No named invariants found.
//! - `use` and `type` are Rust keywords — kept as raw identifiers `r#use`/`r#type`,
//!   matching this crate's `Identifier`/`TypeError` convention.
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

/// The FHIR `Address` complex data type: a postal address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    r#use: Option<Primitive<Code>>,
    r#type: Option<Primitive<Code>>,
    text: Option<Primitive<FhirString>>,
    line: Vec<Primitive<FhirString>>,
    city: Option<Primitive<FhirString>>,
    district: Option<Primitive<FhirString>>,
    state: Option<Primitive<FhirString>>,
    postal_code: Option<Primitive<FhirString>>,
    country: Option<Primitive<FhirString>>,
    period: Option<Period>,
}
