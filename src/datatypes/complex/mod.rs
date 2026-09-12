//! Complex data types defined in the FHIR specification.
//!
//! Reusable multi-element data structures. Only [`Extension`] is implemented; the rest
//! (`period`, `coding`, `quantity`, `codeable_concept`, `identifier`, `reference`) are
//! doc-only stubs — private modules, not part of the public API, kept here so the field
//! shapes and spec citations are already verified when someone picks one up. See
//! `docs/BACKLOG.md` Roadmap for build order and `docs/LLD.md` §4 for why they aren't
//! implemented yet.

mod codeable_concept;
mod coding;
pub mod extension;
mod identifier;
mod period;
mod quantity;
mod reference;

pub use extension::{Extension, ExtensionValue};
