//! The FHIR `Identifier` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Identifier)
//! - Fields: `use` (0..1, `code` — usual | official | temp | secondary | old),
//!   `type` (0..1,
//!   [`CodeableConcept`](crate::datatypes::complex::codeable_concept::CodeableConcept)
//!   — describes the identifier's kind, e.g. MRN), `system` (0..1, `uri` — the
//!   namespace the value is unique within), `value` (0..1, `string`), `period` (0..1,
//!   [`Period`](crate::datatypes::complex::period::Period) — time period the
//!   identifier is valid for), `assigner` (0..1,
//!   [`Reference`](crate::datatypes::complex::reference::Reference) to
//!   `Organization` — who issued it).
//! - No named invariants.
//! - `use`, `system`, `value` are primitive-valued
//!   ([`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern); `type`, `period`, `assigner` are complex types and get no companion.
//! - `use` is a Rust keyword — kept as the raw identifier `r#use` to preserve the exact
//!   FHIR property name, matching this crate's existing convention for the `type` field
//!   (see e.g. `TypeError::r#type` in `src/errors/type.rs`).
//! - `assigner: Box<Reference>` — boxed because [`Reference`] embeds
//!   [`Identifier`] right back (`Reference.identifier`), and two directly-nested structs
//!   referencing each other by value would be infinite-sized. `Reference` mirrors this
//!   with `Box<Identifier>`.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap step 5 (`Identifier` + `Reference`, designed together)
//! before implementing: re-verify this doc against the live spec, then add the
//! constructor, accessors, and serde. Depends on `CodeableConcept` (step 4) and
//! `Period` (step 1); mutually dependent on `Reference` (see above).

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::codeable_concept::CodeableConcept;
use crate::datatypes::complex::period::Period;
use crate::datatypes::complex::reference::Reference;
use crate::datatypes::primitive::Primitive;
use crate::types::{Code, FhirString, Uri};

/// The FHIR `Identifier` complex data type: a business identifier for a resource,
/// scoped to a namespace (`system`) and optionally typed, time-bounded, and attributed
/// to an issuing organization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    r#use: Option<Primitive<Code>>,
    r#type: Option<CodeableConcept>,
    system: Option<Primitive<Uri>>,
    value: Option<Primitive<FhirString>>,
    period: Option<Period>,
    assigner: Option<Box<Reference>>,
}
