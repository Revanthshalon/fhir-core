//! The FHIR `Meta` special-purpose data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/resource.html#Meta)
//! - Fields: `versionId` (0..1, `id`), `lastUpdated` (0..1, `instant`), `source` (0..1,
//!   `uri`), `profile` (0..*, `canonical(StructureDefinition)`), `security` (0..*,
//!   [`Coding`](crate::datatypes::complex::coding::Coding)), `tag` (0..*, `Coding`),
//!   plus `id`/`extension`.
//! - No named invariants found.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `Coding`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::coding::Coding;
use crate::datatypes::primitive::Primitive;
use crate::types::{Canonical, Id, Instant, Uri};

/// The FHIR `Meta` special-purpose data type: version-independent metadata about a
/// resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta {
    version_id: Option<Primitive<Id>>,
    last_updated: Option<Primitive<Instant>>,
    source: Option<Primitive<Uri>>,
    profile: Vec<Primitive<Canonical>>,
    security: Vec<Coding>,
    tag: Vec<Coding>,
}
