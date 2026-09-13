//! Complex data types defined in the FHIR specification.
//!
//! Thirteen types are fully implemented: [`Extension`], [`Period`], [`Coding`],
//! [`Quantity`], [`CodeableConcept`], [`Identifier`], [`Reference`], and the six
//! `Quantity` profiles [`Age`], [`Count`], [`Distance`], [`Duration`],
//! [`MoneyQuantity`], and [`SimpleQuantity`]. `Identifier` and `Reference` are mutually
//! recursive (each embeds the other, boxed) and were designed and built together for
//! that reason. The `Quantity` profiles were deliberately implemented as standalone
//! structs duplicating `Quantity`'s fields rather than newtype wrappers — see
//! `docs/REVIEW_ISSUES.md` ISSUE-002 for the tradeoff — so each can independently
//! evolve its profile-specific invariant without coupling to `Quantity`'s.
//!
//! Every other FHIR R5 complex/metadata/special-purpose type is a **doc-only stub**:
//! `address`, `annotation`, `attachment`, `availability`, `codeable_reference`,
//! `contact_detail`, `contact_point`, `data_requirement`,
//! `dosage`, `element_definition` (intentionally partial — see its module
//! doc), `expression`, `extended_contact_detail`, `human_name`, `meta`,
//! `monetary_component`, `money`, `narrative` (blocked on the
//! unimplemented `xhtml` primitive — see its module doc), `parameter_definition`,
//! `range`, `ratio`, `ratio_range`, `related_artifact`, `sampled_data`, `signature`,
//! `timing`, `trigger_definition`, `usage_context`, and
//! `virtual_service_detail` — private modules (not part of the public API,
//! `#![allow(dead_code)]`'d since nothing constructs them), each with real fields and
//! a module doc citing the spec, kept so the shape is already researched when one is
//! picked up. See `docs/BACKLOG.md` Roadmap and `docs/LLD.md` §4 for why they aren't
//! *implemented* (construction, invariant validation, accessors, serde) yet.

pub mod age;
pub mod codeable_concept;
pub mod coding;
pub mod count;
pub mod distance;
pub mod duration;
pub mod extension;
pub mod identifier;
pub mod money_quantity;
pub mod period;
pub mod quantity;
pub mod reference;
pub mod simple_quantity;

mod address;
mod annotation;
mod attachment;
mod availability;
mod codeable_reference;
mod contact_detail;
mod contact_point;
mod data_requirement;
mod dosage;
mod element_definition;
mod expression;
mod extended_contact_detail;
mod human_name;
mod meta;
mod monetary_component;
mod money;
mod narrative;
mod parameter_definition;
mod range;
mod ratio;
mod ratio_range;
mod related_artifact;
mod sampled_data;
mod signature;
mod timing;
mod trigger_definition;
mod usage_context;
mod virtual_service_detail;

pub use age::Age;
pub use codeable_concept::CodeableConcept;
pub use coding::Coding;
pub use count::Count;
pub use distance::Distance;
pub use duration::Duration;
pub use extension::{Extension, ExtensionValue};
pub use identifier::Identifier;
pub use money_quantity::MoneyQuantity;
pub use period::Period;
pub use quantity::Quantity;
pub use reference::Reference;
pub use simple_quantity::SimpleQuantity;
