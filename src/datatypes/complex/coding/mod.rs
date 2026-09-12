//! The FHIR `Coding` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Coding)
//! - Fields: `system` (0..1, `uri` — the code system), `version` (0..1, `string`),
//!   `code` (0..1, `code`), `display` (0..1, `string`), `userSelected` (0..1,
//!   `boolean` — was this coding chosen directly by a user).
//! - No named invariants.
//! - Every field is primitive-valued, so each is
//!   [`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern (see `Extension`'s module docs).
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap step 2 (`Coding`) before implementing: re-verify this doc
//! against the live spec page, then add the constructor, accessors, and serde. No
//! dependency on other complex types — `CodeableConcept` (step 4) depends on this one.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::primitive::Primitive;
use crate::types::{Boolean, Code, FhirString, Uri};

/// The FHIR `Coding` complex data type: a single code from a code system, optionally
/// with a human-readable display and version/user-selection metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coding {
    system: Option<Primitive<Uri>>,
    version: Option<Primitive<FhirString>>,
    code: Option<Primitive<Code>>,
    display: Option<Primitive<FhirString>>,
    /// FHIR JSON key `userSelected`.
    user_selected: Option<Primitive<Boolean>>,
}
