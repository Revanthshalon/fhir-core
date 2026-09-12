# Backlog

Deferred work items that are known and verified but intentionally not fixed yet,
usually because the real fix needs a design decision (a new type, a breaking
change) rather than a small patch. Each entry should be gone the moment it's
resolved — either fixed for real, or promoted into an ADR/issue if it grows.

## `base64Binary`: LLD/TDD docs describe the R4 grammar, not R5

- **What**: `docs/LLD.md` §3 and `docs/TDD.md` §3 record `base64Binary`'s pattern
  as `(\s*([0-9a-zA-Z+/=]){4}\s*)+` (permits internal whitespace, requires at
  least 4 chars) and list a "with internal whitespace" happy-path test vector.
  That's the **R4** grammar (hl7.org/fhir/R4/datatypes.html#base64Binary). The
  **R5** grammar (hl7.org/fhir/R5/datatypes.html#base64Binary) is
  `(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?` — no
  whitespace, empty string allowed.
- **Why not fixed now**: `Base64Binary::validate` (`src/types/base64/mod.rs`)
  already implements the R5 grammar correctly and is gated behind the `r5`
  feature — only the docs are wrong today. But this crate added an `r4` feature
  flag (`6c5c7ca`) meaning R4 support is an actual near-term target, not just a
  hypothetical. Once R4 primitives are real, `base64Binary` under R4 needs its
  *own* type (or a generic parameterized over grammar) with the R4 regex/
  whitespace rule, since the current `Base64Binary` can't correctly serve both
  specs at once. Fixing the docs alone without deciding that shape first would
  just need re-editing again.
- **Fix shape when picked up**: introduce an R4-specific base64Binary newtype
  (or equivalent), gate it behind `feature = "r4"` following the existing
  primitive pattern, and correct `docs/LLD.md`/`docs/TDD.md` to describe each
  version's grammar against the type that actually implements it.
- **Note**: a single `Base64Binary::validate` cannot just `#[cfg(feature =
  "r4")]`/`#[cfg(feature = "r5")]` two bodies — both features are additive and
  CI builds `--all-features`, so both would be compiled at once (duplicate
  definition). Confirms the type-per-spec shape above, not a cfg split.

## Roadmap: other known-pending work (not yet backlog-worthy, tracked for visibility)

- **`r4` primitives beyond `base64Binary` are unaudited**: only `base64Binary`'s
  R4-vs-R5 grammar difference has been checked against spec so far. The other
  19 primitives were built R5-first; nobody has verified whether their R4
  grammars match. Audit each before claiming real `r4` support.
- **`xhtml` primitive not implemented**: FHIR R5 defines 21 primitive types;
  this crate has 20 (`docs/TECH_DESIGN.md` §3 says "20" — not a doc error, just
  an uncovered type). Low priority — `xhtml` only backs `Narrative.div`.
- **`Base`/`Element`/`Resource` trait hierarchy**: intentionally undesigned —
  see `docs/LLD.md` §4 for why (a prior speculative attempt didn't compile and
  leaked an invariant). Design bottom-up when a real consumer needs it.
- ~~**Remaining complex types**~~ — **done, this section is now historical.**
  All seven originally-scoped complex types (`Extension`, `Period`, `Coding`,
  `Quantity`, `CodeableConcept`, `Identifier`, `Reference`) are implemented,
  each with a validating constructor, accessors, and hand-rolled serde, and
  each wired into `Extension` as an `ExtensionValue` variant. The other ~28
  complex types the spec defines (`Attachment`, `Address`, `ContactPoint`,
  `Money`, ...) are not stubbed or built — new ones get designed only when a
  real consumer needs them (`docs/LLD.md` §4), not speculatively ahead of
  time. Kept below for the field/invariant research trail (dependency order,
  fields verified against hl7.org/fhir/R5/datatypes.html and
  hl7.org/fhir/R5/references.html on 2026-09-12):

  1. ~~**`Period`**~~ — **done.** `start: Option<Primitive<DateTime>>`,
     `end: Option<Primitive<DateTime>>`, plus the `id`/`extension` every
     `Element` carries. Named invariant `per-1` (`start` <= `end`, compared via
     FHIRPath's `lowBoundary()`/`highBoundary()`) exists but is **not**
     validated — `DateTime` has no boundary-expansion/true chronological
     comparison, and its derived `Ord` is plain string order, only correct
     when both values share precision and timezone. Faking `per-1` with that
     would be worse than not checking; see the module's "Known gap" doc.
     Only `ele-1` (at least one of `start`/`end`/`extension`) is enforced.
     Wired into `Extension` as `ExtensionValue::Period` (embeds the whole
     object under `valuePeriod`, no companion split — only primitives get
     that). **Correction**: an earlier version of this entry said Period had
     "no named invariant" — wrong, `per-1` exists at
     hl7.org/fhir/R5/datatypes-definitions.html#Period.end; the earlier check
     only looked at datatypes.html, which doesn't list invariants inline.
  2. ~~**`Coding`**~~ — **done.** `system: Option<Primitive<Uri>>`,
     `version: Option<Primitive<FhirString>>`, `code: Option<Primitive<Code>>`,
     `display: Option<Primitive<FhirString>>`,
     `userSelected: Option<Primitive<Boolean>>`, plus `id`/`extension`. Named
     invariant `cod-1` exists (`code.exists().not() implies
     display.exists().not()`) but is **Warning**-severity (SHOULD), not
     error-severity (SHALL) like `ext-1`/`ele-1`/`per-1` — this crate's
     `ConstraintError` models hard SHALL failures, so `cod-1` is deliberately
     not enforced; a `display`-without-`code` `Coding` is spec-legal, just
     discouraged. Only `ele-1` is validated. Wired into `Extension` as
     `ExtensionValue::Coding` (embeds the whole object under `valueCoding`, no
     companion split). **Correction**: an earlier version of this entry said
     "No named invariants" — wrong, same mistake as the `Period` one above
     (checked datatypes.html only, missed datatypes-definitions.html).
  3. ~~**`Quantity`**~~ — **done.** `value: Option<Primitive<Decimal>>`,
     `comparator: Option<Primitive<Code>>`, `unit: Option<Primitive<FhirString>>`,
     `system: Option<Primitive<Uri>>`, `code: Option<Primitive<Code>>`, plus
     `id`/`extension`. Named invariant `qty-3` (`code.empty() or
     system.exists()`, **error** severity) exists as predicted — unlike
     `per-1`/`cod-1`, this one is a plain presence check with no
     boundary/comparison semantics, so it's fully enforced in
     `Quantity::new`. Only `ele-1` and `qty-3` are validated (no other
     invariants found on datatypes-definitions.html). Wired into `Extension`
     as `ExtensionValue::Quantity` (embeds the whole object under
     `valueQuantity`, no companion split).
  4. ~~**`CodeableConcept`**~~ — **done.** `coding: Vec<Coding>`,
     `text: Option<Primitive<FhirString>>`, plus `id`/`extension`. No named
     invariants for `CodeableConcept` itself (double-checked against
     datatypes-definitions.html) — the only nearby invariant, `cod-1`, belongs
     to `Coding` and is already handled there. Only `ele-1` is validated.
     Wired into `Extension` as `ExtensionValue::CodeableConcept` (embeds the
     whole object under `valueCodeableConcept`, no companion split).
  5. ~~**`Identifier`** and **`Reference`**~~ — **done, built together** since
     they're mutually recursive (`Identifier.assigner: Option<Box<Reference>>`,
     `Reference.identifier: Option<Box<Identifier>>`; `Box` needed since each
     embeds the other by value once — otherwise an infinite-size type):
     - `Identifier`: `use: Option<Primitive<Code>>`,
       `type: Option<CodeableConcept>`, `system: Option<Primitive<Uri>>`,
       `value: Option<Primitive<FhirString>>`, `period: Option<Period>`,
       `assigner: Option<Box<Reference>>`, plus `id`/`extension`. Named
       invariant `ident-1` (`value.exists()`) exists but is
       **Warning**-severity — not enforced, same reasoning as `Coding`'s
       `cod-1`. Only `ele-1` is validated.
     - `Reference`: `reference: Option<Primitive<FhirString>>`,
       `type: Option<Primitive<Uri>>`, `identifier: Option<Box<Identifier>>`,
       `display: Option<Primitive<FhirString>>`, plus `id`/`extension`. Two
       named invariants, both confirmed **error**-severity:
       - `ref-2` — implemented in `Reference::new`: `reference.exists() or
         identifier.exists() or display.exists() or extension.exists()`.
         Note `type` is *not* in the list — stronger than a generic `ele-1`,
         so `ref-2` alone is validated (no separate `ele-1` check needed).
       - `ref-1` — **not implementable at this crate's scope**: requires
         `%rootResource`/`%resource` to validate a local (`#id`) reference
         against sibling `contained` resources, which needs whole-resource-tree
         context this crate doesn't model (no `Resource`/`contained` support
         yet — see the `Base`/`Element`/`Resource` roadmap item above).
         Documented as a known gap in the module doc; a `"#id"` reference is
         accepted without verifying it resolves.
     Both wired into `Extension` as `ExtensionValue::Identifier`/`::Reference`
     (each embeds the whole object, no companion split).
- **`Extension` drops unrecognized `value[x]`** on deserialize (lossy
  round-trip for the ~28 complex types this crate still doesn't model, now
  that `Period`/`Coding`/`Quantity`/`CodeableConcept`/`Identifier`/`Reference`
  are handled). Spec-legal (SHOULD, not MUST) but a real gap — see
  `docs/LLD.md` §4.1 "Known gap". Revisit if/when the next complex type lands,
  or when a caller actually needs lossless round-tripping of an unmodeled
  `value[x]`.
