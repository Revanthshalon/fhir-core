//! The FHIR `MonetaryComponent` metadata type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/metadatatypes.html#MonetaryComponent)
//! - Fields: `type` (1..1, `code` — base | surcharge | deduction | discount | tax |
//!   informational), `code` (0..1,
//!   [`CodeableConcept`](crate::datatypes::complex::codeable_concept::CodeableConcept)),
//!   `factor` (0..1, `decimal`), `amount` (0..1,
//!   [`Money`](crate::datatypes::complex::money::Money)), plus `id`/`extension`.
//! - No named invariants found.
//! - `type` is a Rust keyword — kept as the raw identifier `r#type`, matching this
//!   crate's `Identifier`/`TypeError` convention.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `CodeableConcept` and
//! `Money`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::codeable_concept::CodeableConcept;
use crate::datatypes::complex::money::Money;
use crate::datatypes::primitive::Primitive;
use crate::types::{Code, Decimal, FhirString};

/// The FHIR `MonetaryComponent` metadata type: one component (tax, discount, ...) of
/// a total monetary amount.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonetaryComponent {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    r#type: Option<Primitive<Code>>,
    code: Option<CodeableConcept>,
    factor: Option<Primitive<Decimal>>,
    amount: Option<Money>,
}
