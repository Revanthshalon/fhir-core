//! The FHIR `DataRequirement` metadata type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/metadatatypes.html#DataRequirement)
//! - Top-level fields: `type` (1..1, `code`), `profile` (0..*,
//!   `canonical(StructureDefinition)`), `subject[x]` (0..1, choice of
//!   [`CodeableConcept`](crate::datatypes::complex::codeable_concept::CodeableConcept)
//!   or `Reference(Group)`), `mustSupport` (0..*, `string`), `codeFilter` (0..*,
//!   nested [`CodeFilter`]), `dateFilter` (0..*, nested [`DateFilter`]), `valueFilter`
//!   (0..*, nested [`ValueFilter`]), `limit` (0..1, `positiveInt`), `sort` (0..*,
//!   nested [`Sort`]), plus `id`/`extension`.
//! - `CodeFilter` (`BackboneElement`): `path` (0..1, `string`), `searchParam` (0..1,
//!   `string`), `valueSet` (0..1, `canonical(ValueSet)`), `code` (0..*,
//!   [`Coding`](crate::datatypes::complex::coding::Coding)).
//! - `DateFilter` (`BackboneElement`): `path` (0..1, `string`), `searchParam` (0..1,
//!   `string`), `value[x]` (0..1, choice of `dateTime`,
//!   [`Period`](crate::datatypes::complex::period::Period), or
//!   [`Duration`](crate::datatypes::complex::duration::Duration)).
//! - `ValueFilter` (`BackboneElement`): `path` (0..1, `string`), `searchParam` (0..1,
//!   `string`), `comparator` (0..1, `code`), `value[x]` (0..1, same choice as
//!   `DateFilter`).
//! - `Sort` (`BackboneElement`): `path` (1..1, `string`), `direction` (1..1, `code`).
//! - Named invariants not yet checked (each filter kind historically has a "path xor
//!   searchParam" style rule) — re-verify before implementing.
//! - `type` is a Rust keyword — kept as the raw identifier `r#type`, matching this
//!   crate's `Identifier`/`TypeError` convention.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor(s), accessors, and serde. Depends on `CodeableConcept`,
//! `Reference`, `Coding`, `Period`, `Duration`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::codeable_concept::CodeableConcept;
use crate::datatypes::complex::coding::Coding;
use crate::datatypes::complex::duration::Duration;
use crate::datatypes::complex::period::Period;
use crate::datatypes::complex::reference::Reference;
use crate::datatypes::primitive::Primitive;
use crate::types::{Canonical, Code, DateTime, FhirString, PositiveInt};

/// The value carried by `DataRequirement.subject[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataRequirementSubject {
    /// `subjectCodeableConcept`
    CodeableConcept(CodeableConcept),
    /// `subjectReference` (to `Group`)
    Reference(Reference),
}

/// The value carried by `DataRequirement.dateFilter.value[x]` and
/// `DataRequirement.valueFilter.value[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataRequirementDateValue {
    /// `valueDateTime`
    DateTime(Primitive<DateTime>),
    /// `valuePeriod`
    Period(Period),
    /// `valueDuration`
    Duration(Duration),
}

/// `DataRequirement.codeFilter`, a nested `BackboneElement`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeFilter {
    path: Option<Primitive<FhirString>>,
    search_param: Option<Primitive<FhirString>>,
    value_set: Option<Primitive<Canonical>>,
    code: Vec<Coding>,
}

/// `DataRequirement.dateFilter`, a nested `BackboneElement`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateFilter {
    path: Option<Primitive<FhirString>>,
    search_param: Option<Primitive<FhirString>>,
    value: Option<DataRequirementDateValue>,
}

/// `DataRequirement.valueFilter`, a nested `BackboneElement`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueFilter {
    path: Option<Primitive<FhirString>>,
    search_param: Option<Primitive<FhirString>>,
    comparator: Option<Primitive<Code>>,
    value: Option<DataRequirementDateValue>,
}

/// `DataRequirement.sort`, a nested `BackboneElement`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sort {
    path: Option<Primitive<FhirString>>,
    direction: Option<Primitive<Code>>,
}

/// The FHIR `DataRequirement` metadata type: describes data expected/required by a
/// knowledge module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataRequirement {
    r#type: Option<Primitive<Code>>,
    profile: Vec<Primitive<Canonical>>,
    subject: Option<DataRequirementSubject>,
    must_support: Vec<Primitive<FhirString>>,
    code_filter: Vec<CodeFilter>,
    date_filter: Vec<DateFilter>,
    value_filter: Vec<ValueFilter>,
    limit: Option<Primitive<PositiveInt>>,
    sort: Vec<Sort>,
}
