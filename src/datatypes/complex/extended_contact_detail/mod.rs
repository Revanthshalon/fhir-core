//! The FHIR `ExtendedContactDetail` metadata type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/metadatatypes.html#ExtendedContactDetail)
//! - Fields: `purpose` (0..1,
//!   [`CodeableConcept`](crate::datatypes::complex::codeable_concept::CodeableConcept)),
//!   `name` (0..*, [`HumanName`](crate::datatypes::complex::human_name::HumanName)),
//!   `telecom` (0..*,
//!   [`ContactPoint`](crate::datatypes::complex::contact_point::ContactPoint)),
//!   `address` (0..1, [`Address`](crate::datatypes::complex::address::Address)),
//!   `organization` (0..1,
//!   [`Reference`](crate::datatypes::complex::reference::Reference)), `period` (0..1,
//!   [`Period`](crate::datatypes::complex::period::Period)), plus `id`/`extension`.
//! - No named invariants found.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `CodeableConcept`,
//! `HumanName`, `ContactPoint`, `Address`, `Reference`, `Period`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::address::Address;
use crate::datatypes::complex::codeable_concept::CodeableConcept;
use crate::datatypes::complex::contact_point::ContactPoint;
use crate::datatypes::complex::human_name::HumanName;
use crate::datatypes::complex::period::Period;
use crate::datatypes::complex::reference::Reference;

/// The FHIR `ExtendedContactDetail` metadata type: contact information for a specific
/// purpose, optionally scoped to a time period.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedContactDetail {
    purpose: Option<CodeableConcept>,
    name: Vec<HumanName>,
    telecom: Vec<ContactPoint>,
    address: Option<Address>,
    organization: Option<Reference>,
    period: Option<Period>,
}
