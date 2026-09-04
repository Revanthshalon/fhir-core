//! # fhir-core
//!
//! Core primitives, data types, and error types for Fast Healthcare Interoperability Resources (FHIR).

pub mod datatypes;
pub mod errors;

#[cfg(feature = "r5")]
pub mod types;
