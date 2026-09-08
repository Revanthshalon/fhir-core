//! # fhir-core
//!
//! Core primitives, data types, and error types for Fast Healthcare Interoperability Resources (FHIR).

#[cfg(feature = "r5")]
pub mod datatypes;
pub mod errors;

#[cfg(feature = "r5")]
pub mod types;
