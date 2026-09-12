//! The FHIR `Annotation` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Annotation)
//! - Fields: `author[x]` (0..1, choice of
//!   [`Reference`](crate::datatypes::complex::reference::Reference) or `string`),
//!   `time` (0..1, `dateTime`), `text` (1..1, `markdown`), plus `id`/`extension`.
//! - No named invariants found.
//! - `author[x]` is a genuine FHIR choice type — represented as a local enum
//!   ([`AnnotationAuthor`]), the same pattern `Extension.value[x]`
//!   ([`ExtensionValue`](crate::datatypes::complex::extension::ExtensionValue)) uses.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `Reference`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::reference::Reference;
use crate::datatypes::primitive::Primitive;
use crate::types::{DateTime, FhirString, Markdown};

/// The value carried by `Annotation.author[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotationAuthor {
    /// `authorReference`
    Reference(Reference),
    /// `authorString`
    String(Primitive<FhirString>),
}

/// The FHIR `Annotation` complex data type: a text note with an optional author and
/// timestamp.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    author: Option<AnnotationAuthor>,
    time: Option<Primitive<DateTime>>,
    text: Option<Primitive<Markdown>>,
}
