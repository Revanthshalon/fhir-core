//! Complex data types defined in the FHIR specification.
//!
//! Seven types are fully implemented: [`Extension`], [`Period`], [`Coding`],
//! [`Quantity`], [`CodeableConcept`], [`Identifier`], and [`Reference`]. `Identifier`
//! and `Reference` are mutually recursive (each embeds the other, boxed) and were
//! designed and built together for that reason. This is the originally-scoped set from
//! `docs/BACKLOG.md`'s roadmap.
//!
//! Every other FHIR R5 complex/metadata/special-purpose type is a **doc-only stub**:
//! `age`, `address`, `annotation`, `attachment`, `availability`, `codeable_reference`,
//! `contact_detail`, `contact_point`, `count`, `data_requirement`, `distance`,
//! `dosage`, `duration`, `element_definition` (intentionally partial — see its module
//! doc), `expression`, `extended_contact_detail`, `human_name`, `meta`,
//! `monetary_component`, `money`, `money_quantity`, `narrative` (blocked on the
//! unimplemented `xhtml` primitive — see its module doc), `parameter_definition`,
//! `range`, `ratio`, `ratio_range`, `related_artifact`, `sampled_data`, `signature`,
//! `simple_quantity`, `timing`, `trigger_definition`, `usage_context`, and
//! `virtual_service_detail` — private modules (not part of the public API,
//! `#![allow(dead_code)]`'d since nothing constructs them), each with real fields and
//! a module doc citing the spec, kept so the shape is already researched when one is
//! picked up. See `docs/BACKLOG.md` Roadmap and `docs/LLD.md` §4 for why they aren't
//! *implemented* (construction, invariant validation, accessors, serde) yet.

pub mod codeable_concept;
pub mod coding;
pub mod extension;
pub mod identifier;
pub mod period;
pub mod quantity;
pub mod reference;

mod address;
mod age;
mod annotation;
mod attachment;
mod availability;
mod codeable_reference;
mod contact_detail;
mod contact_point;
mod count;
mod data_requirement;
mod distance;
mod dosage;
mod duration;
mod element_definition;
mod expression;
mod extended_contact_detail;
mod human_name;
mod meta;
mod monetary_component;
mod money;
mod money_quantity;
mod narrative;
mod parameter_definition;
mod range;
mod ratio;
mod ratio_range;
mod related_artifact;
mod sampled_data;
mod signature;
mod simple_quantity;
mod timing;
mod trigger_definition;
mod usage_context;
mod virtual_service_detail;

pub use codeable_concept::CodeableConcept;
pub use coding::Coding;
pub use extension::{Extension, ExtensionValue};
pub use identifier::Identifier;
pub use period::Period;
pub use quantity::Quantity;
pub use reference::Reference;
