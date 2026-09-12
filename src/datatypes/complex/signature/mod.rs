//! The FHIR `Signature` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Signature)
//! - Fields: `type` (0..*, [`Coding`](crate::datatypes::complex::coding::Coding)),
//!   `when` (0..1, `instant`), `who` (0..1,
//!   [`Reference`](crate::datatypes::complex::reference::Reference)), `onBehalfOf`
//!   (0..1, `Reference`), `targetFormat` (0..1, `code`), `sigFormat` (0..1, `code`),
//!   `data` (0..1, `base64Binary`), plus `id`/`extension`.
//! - No named invariants found.
//! - `type` is a Rust keyword — kept as the raw identifier `r#type`, matching this
//!   crate's `Identifier`/`TypeError` convention.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `Coding` and
//! `Reference`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::coding::Coding;
use crate::datatypes::complex::reference::Reference;
use crate::datatypes::primitive::Primitive;
use crate::types::{Base64Binary, Code, FhirString, Instant};

/// The FHIR `Signature` complex data type: a digital signature along with supporting
/// context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    r#type: Vec<Coding>,
    when: Option<Primitive<Instant>>,
    who: Option<Reference>,
    on_behalf_of: Option<Reference>,
    target_format: Option<Primitive<Code>>,
    sig_format: Option<Primitive<Code>>,
    data: Option<Primitive<Base64Binary>>,
}
