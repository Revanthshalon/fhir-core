//! The FHIR `RelatedArtifact` metadata type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/metadatatypes.html#RelatedArtifact)
//! - Fields (cross-checked against a non-pinned continuous-build page, **not** the
//!   frozen R5 5.0.0 page — re-verify field names/cardinality against
//!   hl7.org/fhir/R5/metadatatypes.html before implementing, more so than usual):
//!   `type` (1..1, `code`), `label` (0..1, `string`), `display` (0..1, `string`),
//!   `citation` (0..1, `markdown`), `url` (0..1, `url`), `document` (0..1,
//!   [`Attachment`](crate::datatypes::complex::attachment::Attachment)), `resource`
//!   (0..1, `canonical`), `resourceReference` (0..1,
//!   [`Reference`](crate::datatypes::complex::reference::Reference)), plus
//!   `id`/`extension`.
//! - No named invariants found in the cross-checked source.
//! - `type` is a Rust keyword — kept as the raw identifier `r#type`, matching this
//!   crate's `Identifier`/`TypeError` convention.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live R5 spec
//! (see caveat above), then add the constructor, accessors, and serde. Depends on
//! `Attachment` and `Reference`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::attachment::Attachment;
use crate::datatypes::complex::reference::Reference;
use crate::datatypes::primitive::Primitive;
use crate::types::{Canonical, Code, FhirString, Markdown, Url};

/// The FHIR `RelatedArtifact` metadata type: a related document, citation, or other
/// artifact, and how it relates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelatedArtifact {
    r#type: Option<Primitive<Code>>,
    label: Option<Primitive<FhirString>>,
    display: Option<Primitive<FhirString>>,
    citation: Option<Primitive<Markdown>>,
    url: Option<Primitive<Url>>,
    document: Option<Attachment>,
    resource: Option<Primitive<Canonical>>,
    resource_reference: Option<Reference>,
}
