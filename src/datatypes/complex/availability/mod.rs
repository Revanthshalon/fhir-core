//! The FHIR `Availability` metadata type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/metadatatypes.html#Availability)
//! - Fields: `availableTime` (0..*, nested [`AvailableTime`]), `notAvailableTime`
//!   (0..*, nested [`NotAvailableTime`]), plus `id`/`extension`.
//! - `AvailableTime` (a `BackboneElement`, not a standalone named type): `daysOfWeek`
//!   (0..*, `code`), `allDay` (0..1, `boolean`), `availableStartTime` (0..1, `time`),
//!   `availableEndTime` (0..1, `time`).
//! - `NotAvailableTime` (a `BackboneElement`): `description` (0..1, `string`),
//!   `during` (0..1, [`Period`](crate::datatypes::complex::period::Period)).
//! - Invariant `av-1` (**error** severity, not yet enforced in this stub): "Cannot
//!   include start/end times when selecting all day availability" (FHIRPath:
//!   `allDay.exists().not() or (allDay implies availableStartTime.exists().not() and
//!   availableEndTime.exists().not())`), scoped to each `AvailableTime` entry.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor (enforcing `av-1` per entry), accessors, and serde.
//! Depends on `Period`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::period::Period;
use crate::datatypes::primitive::Primitive;
use crate::types::{Boolean, Code, FhirString, Time};

/// `Availability.availableTime`, a nested `BackboneElement` — carries
/// `modifierExtension` in addition to `id`/`extension`, unlike plain `Element`-derived
/// types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailableTime {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    modifier_extension: Vec<Extension>,
    days_of_week: Vec<Primitive<Code>>,
    all_day: Option<Primitive<Boolean>>,
    available_start_time: Option<Primitive<Time>>,
    available_end_time: Option<Primitive<Time>>,
}

/// `Availability.notAvailableTime`, a nested `BackboneElement` — carries
/// `modifierExtension` in addition to `id`/`extension`, unlike plain `Element`-derived
/// types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotAvailableTime {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    modifier_extension: Vec<Extension>,
    description: Option<Primitive<FhirString>>,
    during: Option<Period>,
}

/// The FHIR `Availability` metadata type: when a person, device, or service is
/// (and isn't) available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Availability {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    available_time: Vec<AvailableTime>,
    not_available_time: Vec<NotAvailableTime>,
}
