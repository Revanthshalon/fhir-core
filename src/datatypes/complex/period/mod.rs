//! The FHIR `Period` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Period)
//! - Fields: `start` (0..1, `dateTime`, inclusive lower boundary), `end` (0..1,
//!   `dateTime` — absent means "no end known or planned", not "ongoing forever").
//! - No named invariant (no `per-1` in the spec text), but `end` (when present) must
//!   not precede `start` (when present) — this crate enforces that as an unnamed
//!   validated-constructor check rather than skipping it for lack of a spec id.
//! - `start`/`end` are primitive-valued, so both are
//!   [`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern (see `Extension`'s module docs).
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap step 1 (`Period`) before implementing: re-verify this
//! doc against the live spec page, then add the validating constructor, accessors,
//! and serde (`Extension`'s hand-rolled `Serialize`/`Deserialize` is the pattern to
//! follow once `Extension` grows an `ExtensionValue::Period` variant).

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::primitive::Primitive;
use crate::types::DateTime;

/// The FHIR `Period` complex data type: a time range with an inclusive start and an
/// optional, possibly-unknown end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Period {
    start: Option<Primitive<DateTime>>,
    end: Option<Primitive<DateTime>>,
}
