# FHIR Data Types Roadmap

Tracks every data type defined by the official FHIR spec — R4
(hl7.org/fhir/R4/datatypes.html, hl7.org/fhir/R4/metadatatypes.html) and R5
(hl7.org/fhir/R5/datatypes.html, hl7.org/fhir/R5/metadatatypes.html) — against this
crate's implementation status. This is the single source of truth for "is type X
done, stubbed, or not started"; `docs/BACKLOG.md` tracks *why* specific gaps are
deferred, `docs/LLD.md`/`docs/TECH_DESIGN.md` describe *how* implemented types are
built.

**Keep this in sync**: when a stub becomes a real implementation (constructor,
validation, accessors, serde, wired into `Extension`/its parent), move its row from
Stub to Implemented and update the summary counts. When a new stub is added, add its
row as Stub. Don't let this drift the way `docs/BACKLOG.md`'s old "34 stubs" count did
(it was already wrong by the time this doc was written — 28, not 34).

Status values:
- **Implemented** — constructor, invariant validation, accessors, and serde exist;
  usable outside the stub-only doc-research stage.
- **Stub** — a private module with real fields and a module doc citing the spec, but no
  constructor/validation/accessors/serde (`#![allow(dead_code)]`'d).
  See `src/datatypes/complex/mod.rs` module doc for the full stub list.
- **Not started** — no file exists yet.
- **N/A (abstract)** — a spec type that every element inherits from rather than one
  instantiated directly; this crate models it as inline fields (`id`/`extension`/
  `modifier_extension`) on each concrete type rather than a standalone struct.

---

## Summary

| Category | R5 types | Implemented | Stub | Not started | N/A |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Primitive | 21 | 20 | — | 1 (`xhtml`) | — |
| General-purpose | 23 | 11 | 12 | — | — |
| Metadata | 11 | 0 | 11 | — | — |
| Special-purpose | 7 | 2 | 5 | — | — |
| **Total** | **62** | **33** | **28** | **1** | — |

(`BackboneElement` is abstract in both R4 and R5 and isn't counted above — see its row
in the General-purpose table.)

---

## Primitive Types

R5 defines 21 primitives (20 in the spec's own "Primitive Types" table, plus `xhtml`,
which the spec pages file under "Special-Purpose Data Types" despite it behaving as a
primitive — this crate follows `docs/BACKLOG.md`'s existing framing and counts it here
as the 21st primitive). R4 has 20 — everything except `integer64`, which R5 added.

| Type | R4 | R5 | Status | Notes |
| :--- | :-: | :-: | :--- | :--- |
| `base64Binary` | ✅ | ✅ | Implemented | R5 grammar only (no internal whitespace). R4 permits internal whitespace — a genuinely different grammar, not yet given its own type. Tracked in `docs/BACKLOG.md`. |
| `boolean` | ✅ | ✅ | Implemented | |
| `canonical` | ✅ | ✅ | Implemented | |
| `code` | ✅ | ✅ | Implemented | |
| `date` | ✅ | ✅ | Implemented | |
| `dateTime` | ✅ | ✅ | Implemented | |
| `decimal` | ✅ | ✅ | Implemented | Serializes via `f64` — documented, deliberate precision tradeoff (`REVIEW_ISSUES.md` ISSUE-006). |
| `id` | ✅ | ✅ | Implemented | |
| `instant` | ✅ | ✅ | Implemented | |
| `integer` | ✅ | ✅ | Implemented | |
| `integer64` | — | ✅ | Implemented | R5-only; no R4 equivalent exists to conflict with. |
| `markdown` | ✅ | ✅ | Implemented | |
| `oid` | ✅ | ✅ | Implemented | |
| `positiveInt` | ✅ | ✅ | Implemented | |
| `string` | ✅ | ✅ | Implemented | Rust type `FhirString` (`string` is a reserved word). |
| `time` | ✅ | ✅ | Implemented | |
| `unsignedInt` | ✅ | ✅ | Implemented | |
| `uri` | ✅ | ✅ | Implemented | |
| `url` | ✅ | ✅ | Implemented | |
| `uuid` | ✅ | ✅ | Implemented | |
| `xhtml` | ✅ | ✅ | Not started | Blocks `Narrative.div` — see `Narrative` below and `docs/BACKLOG.md`. |

**R4/R5 divergence already known, not yet resolved** (`REVIEW_ISSUES.md` ISSUE-009,
`docs/BACKLOG.md`): a single type can't correctly serve both specs where the grammar
differs (`base64Binary`) or the wrapped type differs (`Attachment.size`:
`unsignedInt` in R4, `integer64` in R5). The 18 primitives other than `base64Binary`
haven't been individually re-audited for R4-vs-R5 grammar differences — only assumed
identical.

---

## General-Purpose Structures

| Type | R4 | R5 | Status | Notes |
| :--- | :-: | :-: | :--- | :--- |
| `Identifier` | ✅ | ✅ | Implemented | Mutually recursive with `Reference`. |
| `HumanName` | ✅ | ✅ | Stub | |
| `Address` | ✅ | ✅ | Stub | |
| `ContactPoint` | ✅ | ✅ | Stub | |
| `Timing` | ✅ | ✅ | Stub | `tim-1`..`tim-10` invariants confirmed (`REVIEW_ISSUES.md` ISSUE-003), not yet enforced. |
| `Quantity` | ✅ | ✅ | Implemented | `PartialOrd` via comparator-aware interval logic (`REVIEW_ISSUES.md` ISSUE-011). |
| `SimpleQuantity` | ✅ | ✅ | Implemented | `Quantity` profile — standalone struct, not a newtype (ISSUE-002). `sqty-1` enforced structurally (no `comparator` field). |
| `Attachment` | ✅ | ✅ | Stub | `size` is `unsignedInt` (R4) vs `integer64` (R5) — a real per-version type difference, not just gating. |
| `Range` | ✅ | ✅ | Stub | `rng-2` invariant confirmed. |
| `Period` | ✅ | ✅ | Implemented | `per-1` (`start <= end`) confirmed but not enforced — no true chronological `DateTime` comparison exists yet. |
| `Ratio` | ✅ | ✅ | Stub | `rat-1` invariant confirmed. |
| `RatioRange` | — | ✅ | Stub | R5-only. |
| `CodeableConcept` | ✅ | ✅ | Implemented | |
| `Coding` | ✅ | ✅ | Implemented | `cod-1` is Warning-severity, deliberately not enforced (only SHALL invariants are). |
| `SampledData` | ✅ | ✅ | Stub | `sdd-1` invariant confirmed. |
| `Age` | ✅ | ✅ | Implemented | `Quantity` profile. `age-1` enforced. |
| `Distance` | ✅ | ✅ | Implemented | `Quantity` profile. `dis-1` enforced. |
| `Duration` | ✅ | ✅ | Implemented | `Quantity` profile. `drt-1` enforced. |
| `Count` | ✅ | ✅ | Implemented | `Quantity` profile. `cnt-3` enforced. |
| `Money` | ✅ | ✅ | Stub | |
| `MoneyQuantity` | ✅ | ✅ | Implemented | `Quantity` profile. `mtqy-1` enforced. Spec favors plain `Money` for new content; kept for back-compat. |
| `Annotation` | ✅ | ✅ | Stub | |
| `Signature` | ✅ | ✅ | Stub | |
| `BackboneElement` | ✅ | ✅ | N/A (abstract) | Modeled per-type as inline `id`/`extension`/`modifier_extension` fields on each nested backbone struct (e.g. `Timing`'s `TimingRepeat`), not as a standalone type. |

---

## Metadata Types

None implemented yet — all 11 R5 metadata types (10 shared with R4, plus one R4-only
type correctly *not* carried forward) are still stubs or intentionally absent.

| Type | R4 | R5 | Status | Notes |
| :--- | :-: | :-: | :--- | :--- |
| `ContactDetail` | ✅ | ✅ | Stub | |
| `Contributor` | ✅ | — | Not implemented, not stubbed | Removed after R4 (confirmed via metadatatypes-definitions.html) — correctly excluded, not an oversight. |
| `DataRequirement` | ✅ | ✅ | Stub | |
| `ParameterDefinition` | ✅ | ✅ | Stub | Some fields cross-checked against a non-pinned continuous-build page, not the frozen R5 5.0.0 page — re-verify before implementing. |
| `RelatedArtifact` | ✅ | ✅ | Stub | Same non-pinned-page caveat as `ParameterDefinition`. |
| `TriggerDefinition` | ✅ | ✅ | Stub | Same non-pinned-page caveat. |
| `UsageContext` | ✅ | ✅ | Stub | |
| `Expression` | ✅ | ✅ | Stub | Same non-pinned-page caveat. |
| `ExtendedContactDetail` | — | ✅ | Stub | R5-only. |
| `VirtualServiceDetail` | — | ✅ | Stub | R5-only. |
| `Availability` | — | ✅ | Stub | R5-only. |
| `MonetaryComponent` | — | ✅ | Stub | R5-only. |

---

## Special-Purpose Data Types

| Type | R4 | R5 | Status | Notes |
| :--- | :-: | :-: | :--- | :--- |
| `Reference` | ✅ | ✅ | Implemented | Mutually recursive with `Identifier`. `ref-2` enforced; `ref-1` not implementable at this crate's scope (needs whole-resource-tree `%rootResource`/`contained` context this crate doesn't model). |
| `Narrative` | ✅ | ✅ | Stub, blocked | Needs the unimplemented `xhtml` primitive for `div`; currently uses `FhirString` as an explicitly-flagged placeholder. |
| `Extension` | ✅ | ✅ | Implemented | Hosts every other implemented complex type as an `ExtensionValue` variant. |
| `Meta` | ✅ | ✅ | Stub | |
| `ElementDefinition` | ✅ | ✅ | Stub, intentionally partial | ~20 of ~50 fields, given its exceptional size and profiling-not-instance-data domain. One field (`modifierExtension`) flagged for re-verification — a fetch claimed it wasn't listed despite confirmed `BackboneType` parentage. |
| `Dosage` | ✅ | ✅ | Stub | |
| `CodeableReference` | — | ✅ | Stub | R5-only. |

(`xhtml` is tracked under Primitive Types above, matching this crate's existing
convention, even though the HL7 page files it under this category.)

---

## Where this feeds back

- Picking up any **Stub** row: re-verify its invariants directly against
  hl7.org/fhir/R5/datatypes-definitions.html or metadatatypes-definitions.html (not
  from memory — see `REVIEW_ISSUES.md` ISSUE-003 for what happens when that's
  skipped), implement constructor + validation + accessors + serde following the
  pattern in `src/datatypes/complex/quantity/mod.rs`, then wire it into `Extension` as
  an `ExtensionValue` variant and flip its row here to Implemented.
- **R4 support** is currently theoretical for every implemented type except
  `base64Binary` (where the divergence is merely *documented*, not yet resolved) — see
  `REVIEW_ISSUES.md` ISSUE-009 for the additive-feature-flags architecture problem
  that has to be solved before "R4 implemented" can mean anything stronger than "R5
  code compiles under `--features r4`".
