//! The FHIR `ElementDefinition` special-purpose data type (**intentionally partial
//! stub** — see below).
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/elementdefinition.html#ElementDefinition)
//! `ElementDefinition` is exceptionally large (~50 fields) and belongs to a different
//! domain than the instance data types this crate otherwise models — it defines the
//! shape of a `StructureDefinition`/profile, not clinical/administrative data. Unlike
//! every other stub in this module, this one only lists a representative subset of
//! scalar fields (verified against elementdefinition-definitions.html):
//!
//! `path` (1..1, `string`), `representation` (0..*, `code`), `sliceName` (0..1,
//! `string`), `sliceIsConstraining` (0..1, `boolean`), `label` (0..1, `string`),
//! `short` (0..1, `string`), `definition` (0..1, `markdown`), `comment` (0..1,
//! `markdown`), `requirements` (0..1, `markdown`), `alias` (0..*, `string`), `min`
//! (0..1, `unsignedInt`), `max` (0..1, `string`), `base` (0..1, nested [`ElementBase`]:
//! `path`/`min`/`max`), `contentReference` (0..1, `uri`), `mustSupport` (0..1,
//! `boolean`), `isModifier` (0..1, `boolean`), `isModifierReason` (0..1, `string`),
//! `isSummary` (0..1, `boolean`), `maxLength` (0..1, `integer`), `condition` (0..*,
//! `id`), plus `id`/`extension`.
//!
//! **Deliberately omitted** from this stub (add when real implementation is
//! undertaken, each needs its own spec fetch first): `slicing`, `type` (a list of
//! `ElementDefinitionType`, each with `code`/`profile`/`targetProfile`/...),
//! `defaultValue[x]`, `meaningWhenMissing`, `orderMeaning`, `fixed[x]`, `pattern[x]`,
//! `example` (list of `label`/`value[x]`), `minValue[x]`, `maxValue[x]`, `constraint`
//! (list of invariant definitions — `key`/`severity`/`human`/`expression`/...),
//! `mustHaveValue`, `valueAlternatives`, `binding` (terminology binding strength +
//! valueset), `mapping` (list of `identity`/`language`/`map`/`comment`).
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet, and more
//! incomplete than the other stubs in this module (see above). See
//! `docs/BACKLOG.md` Roadmap.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::primitive::Primitive;
use crate::types::{Boolean, Code, FhirString, Id, Integer, Markdown, UnsignedInt, Uri};

/// `ElementDefinition.base`, a nested `BackboneElement` — carries `modifierExtension`
/// in addition to `id`/`extension`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementBase {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    modifier_extension: Vec<Extension>,
    path: Option<Primitive<FhirString>>,
    min: Option<Primitive<UnsignedInt>>,
    max: Option<Primitive<FhirString>>,
}

/// The FHIR `ElementDefinition` special-purpose data type: defines the constraints on
/// an element in a resource or data type (used by `StructureDefinition`). See the
/// module docs — this stub covers only a representative subset of fields.
///
/// Includes `modifier_extension`: `ElementDefinition`'s base type is `BackboneType`
/// (confirmed via elementdefinition-definitions.html), which — like `BackboneElement`
/// — carries `modifierExtension` in addition to the `id`/`extension` every `Element`
/// has. A secondary summarized fetch claimed no `modifierExtension` field is listed;
/// re-verify directly against the field table before implementing, given that
/// contradiction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementDefinition {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    modifier_extension: Vec<Extension>,
    path: Option<Primitive<FhirString>>,
    representation: Vec<Primitive<Code>>,
    slice_name: Option<Primitive<FhirString>>,
    slice_is_constraining: Option<Primitive<Boolean>>,
    label: Option<Primitive<FhirString>>,
    short: Option<Primitive<FhirString>>,
    definition: Option<Primitive<Markdown>>,
    comment: Option<Primitive<Markdown>>,
    requirements: Option<Primitive<Markdown>>,
    alias: Vec<Primitive<FhirString>>,
    min: Option<Primitive<UnsignedInt>>,
    max: Option<Primitive<FhirString>>,
    base: Option<ElementBase>,
    content_reference: Option<Primitive<Uri>>,
    must_support: Option<Primitive<Boolean>>,
    is_modifier: Option<Primitive<Boolean>>,
    is_modifier_reason: Option<Primitive<FhirString>>,
    is_summary: Option<Primitive<Boolean>>,
    max_length: Option<Primitive<Integer>>,
    condition: Vec<Primitive<Id>>,
}
