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
