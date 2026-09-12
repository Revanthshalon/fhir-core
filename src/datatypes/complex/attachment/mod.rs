//! The FHIR `Attachment` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Attachment)
//! - Fields: `contentType` (0..1, `code`), `language` (0..1, `code`), `data` (0..1,
//!   `base64Binary`), `url` (0..1, `url`), `size` (0..1, `integer64`), `hash` (0..1,
//!   `base64Binary`), `title` (0..1, `string`), `creation` (0..1, `dateTime`), `height`
//!   (0..1, `positiveInt`), `width` (0..1, `positiveInt`), `frames` (0..1,
//!   `positiveInt`), `duration` (0..1, `decimal`), `pages` (0..1, `positiveInt`), plus
//!   `id`/`extension`.
//! - Invariant `att-1` (**error** severity, not yet enforced in this stub): "If the
//!   Attachment has data, it SHALL have a contentType."
//! - **Feature-gating trap, and it's bigger than just gating**: `size` is `integer64`
//!   in R5, but confirmed `unsignedInt` in R4 (hl7.org/fhir/R4/datatypes.html#Attachment)
//!   — a genuinely different primitive type per version, not merely "absent under r4"
//!   the way this stub currently models it. This is the exact same "one type can't
//!   serve two specs" problem `docs/BACKLOG.md`'s `base64Binary` entry already tracks,
//!   generalized. Implementing `Attachment.size` for real needs that R4/R5-versioning
//!   decision made first (see `docs/BACKLOG.md` roadmap and `REVIEW_ISSUES.md`
//!   ISSUE-009) — gating `size` behind `#[cfg(feature = "r5")]` as done below is only
//!   accurate for the R5 half; it is not yet a real R4 `Attachment.size: unsignedInt`.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor (enforcing `att-1`), accessors, and serde.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::primitive::Primitive;
#[cfg(feature = "r5")]
use crate::types::Integer64;
use crate::types::{Base64Binary, Code, DateTime, Decimal, FhirString, PositiveInt, Url};

/// The FHIR `Attachment` complex data type: content in a defined format and encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    content_type: Option<Primitive<Code>>,
    language: Option<Primitive<Code>>,
    data: Option<Primitive<Base64Binary>>,
    url: Option<Primitive<Url>>,
    #[cfg(feature = "r5")]
    size: Option<Primitive<Integer64>>,
    hash: Option<Primitive<Base64Binary>>,
    title: Option<Primitive<FhirString>>,
    creation: Option<Primitive<DateTime>>,
    height: Option<Primitive<PositiveInt>>,
    width: Option<Primitive<PositiveInt>>,
    frames: Option<Primitive<PositiveInt>>,
    duration: Option<Primitive<Decimal>>,
    pages: Option<Primitive<PositiveInt>>,
}
