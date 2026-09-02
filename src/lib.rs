//! # fhir-core
//!
//! Core primitives, data types, and error types for Fast Healthcare Interoperability Resources (FHIR).

pub mod errors;
pub mod primitives;

#[cfg(feature = "r5")]
pub mod r5;
