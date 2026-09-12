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
//! - **Feature-gating trap**: `size` is `integer64`, which
//!   [`Integer64`](crate::types::Integer64) only exists under `feature = "r5"` (added
//!   in R5, not R4 — same reasoning as `Extension::Integer64`). When implementing,
//!   gate the `size` field, every accessor/serialize/deserialize arm for it, and its
//!   import behind `#[cfg(feature = "r5")]` individually — do not gate the whole
//!   `Attachment` type, since every other field is R4-and-R5-common.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor (enforcing `att-1`), accessors, and serde.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::primitive::Primitive;
#[cfg(feature = "r5")]
use crate::types::Integer64;
use crate::types::{Base64Binary, Code, DateTime, Decimal, FhirString, PositiveInt, Url};

/// The FHIR `Attachment` complex data type: content in a defined format and encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
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
