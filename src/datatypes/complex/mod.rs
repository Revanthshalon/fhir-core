//! Complex data types defined in the FHIR specification.
//!
//! Reusable multi-element data structures. [`Extension`], [`Period`], [`Coding`],
//! [`Quantity`], and [`CodeableConcept`] are implemented; the rest (`identifier`,
//! `reference`) are doc-only stubs — private modules, not part of the public API, kept
//! here so the field shapes and spec citations are already verified when someone picks
//! one up. See `docs/BACKLOG.md` Roadmap for build order and `docs/LLD.md` §4 for why
//! they aren't implemented yet.

pub mod codeable_concept;
pub mod coding;
pub mod extension;
mod identifier;
pub mod period;
pub mod quantity;
mod reference;

pub use codeable_concept::CodeableConcept;
pub use coding::Coding;
pub use extension::{Extension, ExtensionValue};
pub use period::Period;
pub use quantity::Quantity;
