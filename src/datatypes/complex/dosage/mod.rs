//! The FHIR `Dosage` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/dosage.html#Dosage)
//! - Top-level fields: `sequence` (0..1, `integer`), `text` (0..1, `string`),
//!   `additionalInstruction` (0..*,
//!   [`CodeableConcept`](crate::datatypes::complex::codeable_concept::CodeableConcept)),
//!   `patientInstruction` (0..1, `string`), `timing` (0..1,
//!   [`Timing`](crate::datatypes::complex::timing::Timing)), `asNeeded` (0..1,
//!   `boolean`), `asNeededFor` (0..*, `CodeableConcept`), `site` (0..1,
//!   `CodeableConcept`), `route` (0..1, `CodeableConcept`), `method` (0..1,
//!   `CodeableConcept`), `doseAndRate` (0..*, nested [`DoseAndRate`]),
//!   `maxDosePerPeriod` (0..*, [`Ratio`](crate::datatypes::complex::ratio::Ratio)),
//!   `maxDosePerAdministration` (0..1,
//!   [`SimpleQuantity`](crate::datatypes::complex::simple_quantity::SimpleQuantity)),
//!   `maxDosePerLifetime` (0..1, `SimpleQuantity`), plus `id`/`extension`.
//! - `DoseAndRate` (a `BackboneElement`, not a standalone named type): `type` (0..1,
//!   `CodeableConcept`), `dose[x]` (0..1, choice of `SimpleQuantity` or
//!   [`Range`](crate::datatypes::complex::range::Range)), `rate[x]` (0..1, choice of
//!   `Ratio`, `Range`, or `SimpleQuantity`).
//! - Invariant `dos-1` (**error** severity, not yet enforced in this stub):
//!   "AsNeededFor can only be set if AsNeeded is empty or true."
//! - `type` is a Rust keyword — kept as the raw identifier `r#type` on `DoseAndRate`,
//!   matching this crate's `Identifier`/`TypeError` convention.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor (enforcing `dos-1`), accessors, and serde. Depends on
//! `CodeableConcept`, `Timing`, `Ratio`, `SimpleQuantity`, `Range`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::codeable_concept::CodeableConcept;
use crate::datatypes::complex::range::Range;
use crate::datatypes::complex::ratio::Ratio;
use crate::datatypes::complex::simple_quantity::SimpleQuantity;
use crate::datatypes::complex::timing::Timing;
use crate::datatypes::primitive::Primitive;
use crate::types::{Boolean, FhirString, Integer};

/// The value carried by `Dosage.doseAndRate.dose[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoseAndRateDose {
    /// `doseRange`
    Range(Box<Range>),
    /// `doseQuantity` (a `SimpleQuantity`)
    Quantity(Box<SimpleQuantity>),
}

/// The value carried by `Dosage.doseAndRate.rate[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoseAndRateRate {
    /// `rateRatio`
    Ratio(Ratio),
    /// `rateRange`
    Range(Range),
    /// `rateQuantity` (a `SimpleQuantity`)
    Quantity(SimpleQuantity),
}

/// `Dosage.doseAndRate`, a nested `BackboneElement`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoseAndRate {
    r#type: Option<CodeableConcept>,
    dose: Option<DoseAndRateDose>,
    rate: Option<DoseAndRateRate>,
}

/// The FHIR `Dosage` complex data type: how a medication should be administered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dosage {
    sequence: Option<Primitive<Integer>>,
    text: Option<Primitive<FhirString>>,
    additional_instruction: Vec<CodeableConcept>,
    patient_instruction: Option<Primitive<FhirString>>,
    timing: Option<Timing>,
    as_needed: Option<Primitive<Boolean>>,
    as_needed_for: Vec<CodeableConcept>,
    site: Option<CodeableConcept>,
    route: Option<CodeableConcept>,
    method: Option<CodeableConcept>,
    dose_and_rate: Vec<DoseAndRate>,
    max_dose_per_period: Vec<Ratio>,
    max_dose_per_administration: Option<SimpleQuantity>,
    max_dose_per_lifetime: Option<SimpleQuantity>,
}
