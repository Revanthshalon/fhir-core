//! # fhir-core
//!
//! Core primitives, data types, and error types for Fast Healthcare Interoperability Resources (FHIR).

#![forbid(unsafe_code)]

#[cfg(feature = "r5")]
pub mod datatypes;
pub mod errors;

#[cfg(any(feature = "r4", feature = "r5"))]
pub mod types;
