//! The FHIR `SampledData` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#SampledData)
//! - Fields: `origin` (1..1,
//!   [`SimpleQuantity`](crate::datatypes::complex::simple_quantity::SimpleQuantity) —
//!   zero-measurement baseline), `interval` (0..1, `decimal`), `intervalUnit` (1..1,
//!   `code`), `factor` (0..1, `decimal`), `lowerLimit` (0..1, `decimal`), `upperLimit`
//!   (0..1, `decimal`), `dimensions` (1..1, `positiveInt`), `codeMap` (0..1,
//!   `canonical`), `offsets` (0..1, `string`), `data` (0..1, `string`), plus
//!   `id`/`extension`.
//! - Invariant `sdd-1` (**error** severity, confirmed via datatypes-definitions.html —
//!   corrects an earlier draft of this doc that guessed the id as `sdata-1` and the
//!   rule as a length-matching constraint, not yet enforced in this stub): "A
//!   SampledData SHALL have either an interval and offsets but not both" (FHIRPath:
//!   `interval.exists().not() xor offsets.exists().not()`) — a plain presence/XOR
//!   check, no boundary-expansion support needed, so fully implementable.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `SimpleQuantity`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::simple_quantity::SimpleQuantity;
use crate::datatypes::primitive::Primitive;
use crate::types::{Canonical, Code, Decimal, FhirString, PositiveInt};

/// The FHIR `SampledData` complex data type: a series of measurements taken at
/// regular intervals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampledData {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    origin: Option<SimpleQuantity>,
    interval: Option<Primitive<Decimal>>,
    interval_unit: Option<Primitive<Code>>,
    factor: Option<Primitive<Decimal>>,
    lower_limit: Option<Primitive<Decimal>>,
    upper_limit: Option<Primitive<Decimal>>,
    dimensions: Option<Primitive<PositiveInt>>,
    code_map: Option<Primitive<Canonical>>,
    offsets: Option<Primitive<FhirString>>,
    data: Option<Primitive<FhirString>>,
}
