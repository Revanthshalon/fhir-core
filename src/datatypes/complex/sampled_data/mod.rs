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
//! - Named invariants not yet checked against datatypes-definitions.html — re-verify
//!   before implementing (this type has historically had a `sdata-1`-style rule about
//!   `data`/`offsets` length matching `dimensions`; confirm the exact wording first).
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `SimpleQuantity`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::simple_quantity::SimpleQuantity;
use crate::datatypes::primitive::Primitive;
use crate::types::{Canonical, Code, Decimal, FhirString, PositiveInt};

/// The FHIR `SampledData` complex data type: a series of measurements taken at
/// regular intervals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampledData {
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
