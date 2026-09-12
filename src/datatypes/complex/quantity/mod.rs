//! The FHIR `Quantity` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Quantity)
//! - Fields: `value` (0..1, `decimal`), `comparator` (0..1, `code` — one of `<`, `<=`,
//!   `>=`, `>`, `ad`, qualifying how `value` should be understood), `unit` (0..1,
//!   `string` — human-readable unit), `system` (0..1, `uri` — the unit code system,
//!   e.g. UCUM), `code` (0..1, `code` — computer-processable unit in `system`).
//! - No named invariants found on the datatypes page as of this writing, but that was
//!   *not* cross-checked against hl7.org/fhir/R5/datatypes-definitions.html#Quantity —
//!   do that before assuming there really are none (e.g. a `code`-requires-`system`
//!   coupling rule would be typical for this shape and easy to miss).
//! - Every field is primitive-valued, so each is
//!   [`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern (see `Extension`'s module docs).
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap step 3 (`Quantity`) before implementing: re-verify this
//! doc (especially the invariant question above) against the live spec, then add the
//! constructor, accessors, and serde. No dependency on other complex types.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::primitive::Primitive;
use crate::types::{Code, Decimal, FhirString, Uri};

/// The FHIR `Quantity` complex data type: a measured amount with an optional unit,
/// unit-coding system, and comparator for open-ended/approximate values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quantity {
    value: Option<Primitive<Decimal>>,
    comparator: Option<Primitive<Code>>,
    unit: Option<Primitive<FhirString>>,
    system: Option<Primitive<Uri>>,
    code: Option<Primitive<Code>>,
}
