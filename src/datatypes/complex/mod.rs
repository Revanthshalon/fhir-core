//! Complex data types defined in the FHIR specification.
//!
//! Reusable multi-element data structures. Currently only [`Extension`] — the others
//! (`Coding`, `CodeableConcept`, `Quantity`, `Period`, `Identifier`, ...) are added when
//! a concrete consumer needs them, per `docs/LLD.md` §4.

pub mod extension;

pub use extension::{Extension, ExtensionValue};
