//! The FHIR `TriggerDefinition` metadata type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/metadatatypes.html#TriggerDefinition)
//! - Fields (cross-checked against a non-pinned continuous-build page, **not** the
//!   frozen R5 5.0.0 page — re-verify against hl7.org/fhir/R5/metadatatypes.html
//!   before implementing, more so than usual): `type` (1..1, `code` — named-event |
//!   periodic | data-changed | data-added | data-modified | data-removed |
//!   data-accessed | data-access-ended | subscription-topic), `name` (0..1,
//!   `string`), `code` (0..1,
//!   [`CodeableConcept`](crate::datatypes::complex::codeable_concept::CodeableConcept)),
//!   `subscriptionTopic` (0..1, `canonical(SubscriptionTopic)`), `timing[x]` (0..1,
//!   choice of [`Timing`](crate::datatypes::complex::timing::Timing), `date`, or
//!   `dateTime`), `data` (0..*,
//!   [`DataRequirement`](crate::datatypes::complex::data_requirement::DataRequirement)),
//!   `condition` (0..1,
//!   [`Expression`](crate::datatypes::complex::expression::Expression)), plus
//!   `id`/`extension`.
//! - Constraint (severity not confirmed from the cross-checked source): `timing[x]`
//!   and `data` are mutually exclusive by trigger `type`, not a structural SHALL —
//!   re-verify.
//! - `type` is a Rust keyword — kept as the raw identifier `r#type`, matching this
//!   crate's `Identifier`/`TypeError` convention.
//! - `timing[x]` is a genuine FHIR choice type — represented as a local enum
//!   ([`TriggerTiming`]).
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live R5 spec
//! (see caveat above), then add the constructor, accessors, and serde. Depends on
//! `CodeableConcept`, `Timing`, `DataRequirement`, `Expression`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::codeable_concept::CodeableConcept;
use crate::datatypes::complex::data_requirement::DataRequirement;
use crate::datatypes::complex::expression::Expression;
use crate::datatypes::complex::timing::Timing;
use crate::datatypes::primitive::Primitive;
use crate::types::{Canonical, Code, Date, DateTime, FhirString};

/// The value carried by `TriggerDefinition.timing[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerTiming {
    /// `timingTiming`
    Timing(Box<Timing>),
    /// `timingDate`
    Date(Primitive<Date>),
    /// `timingDateTime`
    DateTime(Primitive<DateTime>),
}

/// The FHIR `TriggerDefinition` metadata type: describes an event that can trigger a
/// module's execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerDefinition {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    r#type: Option<Primitive<Code>>,
    name: Option<Primitive<FhirString>>,
    code: Option<CodeableConcept>,
    subscription_topic: Option<Primitive<Canonical>>,
    timing: Option<TriggerTiming>,
    data: Vec<DataRequirement>,
    condition: Option<Expression>,
}
