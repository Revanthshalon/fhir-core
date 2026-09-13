//! The FHIR `Timing` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Timing)
//! - Top-level fields: `event` (0..*, `dateTime`), `code` (0..1,
//!   [`CodeableConcept`](crate::datatypes::complex::codeable_concept::CodeableConcept)),
//!   `repeat` (0..1, nested [`TimingRepeat`]), plus `id`/`extension`.
//! - `TimingRepeat` (a `BackboneElement`, not a standalone named type): `bounds[x]`
//!   (0..1, choice of [`Duration`](crate::datatypes::complex::duration::Duration),
//!   [`Range`](crate::datatypes::complex::range::Range), or
//!   [`Period`](crate::datatypes::complex::period::Period)), `count` (0..1,
//!   `positiveInt`), `countMax` (0..1, `positiveInt`), `duration` (0..1, `decimal`),
//!   `durationMax` (0..1, `decimal`), `durationUnit` (0..1, `code`), `frequency`
//!   (0..1, `positiveInt`), `frequencyMax` (0..1, `positiveInt`), `period` (0..1,
//!   `decimal`), `periodMax` (0..1, `decimal`), `periodUnit` (0..1, `code`),
//!   `dayOfWeek` (0..*, `code`), `timeOfDay` (0..*, `time`), `when` (0..*, `code`),
//!   `offset` (0..1, `unsignedInt`).
//! - Confirmed invariants on `TimingRepeat` (hl7.org/fhir/R5/datatypes-definitions.html#Timing.repeat),
//!   all **error** severity ("Rule"), none yet enforced in this stub — note there is no
//!   `tim-3`:
//!   - `tim-1`: `duration.empty() or durationUnit.exists()`
//!   - `tim-2`: `period.empty() or periodUnit.exists()`
//!   - `tim-4`: `duration.exists() implies duration >= 0`
//!   - `tim-5`: `period.exists() implies period >= 0`
//!   - `tim-6`: `periodMax.empty() or period.exists()`
//!   - `tim-7`: `durationMax.empty() or duration.exists()`
//!   - `tim-8`: `countMax.empty() or count.exists()`
//!   - `tim-9`: `offset.empty() or (when.exists() and when.select($this in ('C' | 'CM' | 'CD' | 'CV')).allFalse())`
//!   - `tim-10`: `timeOfDay.empty() or when.empty()`
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `CodeableConcept`,
//! `Duration`, `Range`, `Period`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::codeable_concept::CodeableConcept;
use crate::datatypes::complex::duration::Duration;
use crate::datatypes::complex::period::Period;
use crate::datatypes::complex::range::Range;
use crate::datatypes::primitive::Primitive;
use crate::types::{Code, DateTime, Decimal, FhirString, PositiveInt, Time, UnsignedInt};

/// The value carried by `Timing.repeat.bounds[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimingBounds {
    /// `boundsDuration`
    Duration(Box<Duration>),
    /// `boundsRange`
    Range(Box<Range>),
    /// `boundsPeriod`
    Period(Period),
}

/// `Timing.repeat`, a nested `BackboneElement` — carries `modifierExtension` in
/// addition to `id`/`extension`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingRepeat {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    modifier_extension: Vec<Extension>,
    bounds: Option<TimingBounds>,
    count: Option<Primitive<PositiveInt>>,
    count_max: Option<Primitive<PositiveInt>>,
    duration: Option<Primitive<Decimal>>,
    duration_max: Option<Primitive<Decimal>>,
    duration_unit: Option<Primitive<Code>>,
    frequency: Option<Primitive<PositiveInt>>,
    frequency_max: Option<Primitive<PositiveInt>>,
    period: Option<Primitive<Decimal>>,
    period_max: Option<Primitive<Decimal>>,
    period_unit: Option<Primitive<Code>>,
    day_of_week: Vec<Primitive<Code>>,
    time_of_day: Vec<Primitive<Time>>,
    when: Vec<Primitive<Code>>,
    offset: Option<Primitive<UnsignedInt>>,
}

/// The FHIR `Timing` complex data type: a schedule of events, either explicit or
/// described by a repeating pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timing {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    event: Vec<Primitive<DateTime>>,
    code: Option<CodeableConcept>,
    repeat: Option<TimingRepeat>,
}
