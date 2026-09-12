//! The FHIR `Reference` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/references.html#Reference)
//! - Fields: `reference` (0..1, `string` — a literal reference, e.g.
//!   `"Patient/123"` or a contained `"#id"`), `type` (0..1, `uri` — the expected target
//!   resource type), `identifier` (0..1,
//!   [`Identifier`](crate::datatypes::complex::identifier::Identifier) — used when no
//!   literal reference is available), `display` (0..1, `string` — plain text naming
//!   the target).
//! - Two named invariants:
//!   - `ref-2` (implementable at this crate's scope): `reference.exists() or
//!     identifier.exists() or display.exists() or extension.exists()` — at least one
//!     of the four must be present. Same shape as `Extension::validate_ext1`.
//!   - `ref-1` (**not implementable here**): `reference.exists() implies
//!     (reference.startsWith('#').not() or (reference.substring(1) in
//!     %rootResource.contained.id) or ...)` — validating a local (`#id`) reference
//!     requires whole-resource-tree context (`%rootResource`/`%resource`,
//!     `contained`) this crate doesn't model. Document as a known gap on `Reference`
//!     when implemented; don't fake a partial check.
//! - `reference`, `type`, `display` are primitive-valued
//!   ([`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern); `identifier` is a complex type and gets no companion.
//! - `identifier: Box<Identifier>` — boxed for the same mutual-embedding reason
//!   documented on [`Identifier`]'s `assigner` field.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap step 5 (`Identifier` + `Reference`, designed together)
//! before implementing: re-verify this doc against the live spec, then add the
//! `ref-2`-validating constructor, accessors, and serde, and record the `ref-1` gap
//! wherever this crate tracks known gaps (see `docs/LLD.md` §4.1's `Extension`
//! `value[x]` gap for the pattern).

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::identifier::Identifier;
use crate::datatypes::primitive::Primitive;
use crate::types::{FhirString, Uri};

/// The FHIR `Reference` complex data type: a pointer to another resource, by literal
/// reference string, business identifier, or display text (at least one of the three,
/// or an extension, per `ref-2`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    reference: Option<Primitive<FhirString>>,
    r#type: Option<Primitive<Uri>>,
    identifier: Option<Box<Identifier>>,
    display: Option<Primitive<FhirString>>,
}
