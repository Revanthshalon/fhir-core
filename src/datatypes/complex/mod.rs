//! Complex data types defined in the FHIR specification.
//!
//! Reusable multi-element data structures: [`Extension`], [`Period`], [`Coding`],
//! [`Quantity`], [`CodeableConcept`], [`Identifier`], and [`Reference`]. `Identifier`
//! and `Reference` are mutually recursive (each embeds the other, boxed) and were
//! designed and built together for that reason. This completes the set originally
//! scoped in `docs/BACKLOG.md`'s roadmap — the other ~28 complex types the spec
//! defines (`Quantity`'s siblings `Money`/`Duration`/etc., `Attachment`, `Address`,
//! `ContactPoint`, ...) aren't stubbed or built; see `docs/LLD.md` §4 for why new ones
//! get designed only when a real consumer needs them, not speculatively ahead of time.

pub mod codeable_concept;
pub mod coding;
pub mod extension;
pub mod identifier;
pub mod period;
pub mod quantity;
pub mod reference;

pub use codeable_concept::CodeableConcept;
pub use coding::Coding;
pub use extension::{Extension, ExtensionValue};
pub use identifier::Identifier;
pub use period::Period;
pub use quantity::Quantity;
pub use reference::Reference;
