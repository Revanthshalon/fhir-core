//! The FHIR `UsageContext` metadata type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/metadatatypes.html#UsageContext)
//! - Fields: `code` (1..1, [`Coding`](crate::datatypes::complex::coding::Coding)),
//!   `value[x]` (1..1, choice of
//!   [`CodeableConcept`](crate::datatypes::complex::codeable_concept::CodeableConcept),
//!   [`Quantity`](crate::datatypes::complex::quantity::Quantity),
//!   [`Range`](crate::datatypes::complex::range::Range),
//!   [`Reference`](crate::datatypes::complex::reference::Reference), or `canonical`),
//!   plus `id`/`extension`.
//! - No named invariants found.
//! - `value[x]` is a genuine FHIR choice type — represented as a local enum
//!   ([`UsageContextValue`]), same pattern as `Annotation.author[x]`.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `Coding`,
//! `CodeableConcept`, `Quantity`, `Range`, `Reference`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::codeable_concept::CodeableConcept;
use crate::datatypes::complex::coding::Coding;
use crate::datatypes::complex::quantity::Quantity;
use crate::datatypes::complex::range::Range;
use crate::datatypes::complex::reference::Reference;
use crate::datatypes::primitive::Primitive;
use crate::types::Canonical;

/// The value carried by `UsageContext.value[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsageContextValue {
    /// `valueCodeableConcept`
    CodeableConcept(CodeableConcept),
    /// `valueQuantity`
    Quantity(Quantity),
    /// `valueRange`
    Range(Range),
    /// `valueReference`
    Reference(Reference),
    /// `valueCanonical`
    Canonical(Primitive<Canonical>),
}

/// The FHIR `UsageContext` metadata type: describes a context of use for a
/// conformance/knowledge artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageContext {
    code: Option<Coding>,
    value: Option<UsageContextValue>,
}
