//! The FHIR `ContactDetail` metadata type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/metadatatypes.html#ContactDetail)
//! - Fields: `name` (0..1, `string`), `telecom` (0..*,
//!   [`ContactPoint`](crate::datatypes::complex::contact_point::ContactPoint)), plus
//!   `id`/`extension`.
//! - No named invariants found.
//! - Used by conformance/metadata resources (e.g. `CapabilityStatement.contact`), not
//!   by clinical resources — a different consumer category than the seven types
//!   already implemented in this crate.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `ContactPoint`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::contact_point::ContactPoint;
use crate::datatypes::primitive::Primitive;
use crate::types::FhirString;

/// The FHIR `ContactDetail` metadata type: contact information for a person or
/// organization, used to describe artifact authors etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactDetail {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    name: Option<Primitive<FhirString>>,
    telecom: Vec<ContactPoint>,
}
