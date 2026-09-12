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
- **Remaining complex types** (`Coding`, `CodeableConcept`, `Identifier`,
  `Period`, `Quantity`, `Reference`): only `Extension` exists today. See
  `docs/LLD.md` §4.1 — each becomes real when something needs it, not before.
- **`Extension` drops unrecognized `value[x]`** on deserialize (lossy
  round-trip for the 34 complex types this crate doesn't model yet). Spec-legal
  (SHOULD, not MUST) but a real gap — see `docs/LLD.md` §4.1 "Known gap".
  Revisit once the first complex type lands.
