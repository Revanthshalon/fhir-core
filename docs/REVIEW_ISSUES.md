# Codebase Review & Actionable Issues: `fhir-core`

**Date:** 2026-09-12  
**Target Specification:** HL7 FHIR Release 5 (R5) / Release 4 (R4)  
**Review Scope:** Primitives engine, complex data types, 34 doc-only stubs, error handling, serde protocols, architectural documentation, and backlog.

---

## Executive Summary

A comprehensive architectural and implementation review of `fhir-core` was performed. The crate demonstrates exemplary software engineering fundamentals: zero unsafe code (`#![forbid(unsafe_code)]`), zero non-serde runtime dependencies, zero regex backtracking risk through hand-rolled scanners, and thorough unit testing (692 tests passing).

However, critical issues and technical debt were identified across four main areas:
1. **The 34 Doc-Only Stubs:** Structural flaws where inherited `Element` fields (`id`, `extension`) are missing from struct definitions, missing `modifierExtension` on backbone elements, and anti-pattern field duplication across `Quantity` profiles.
2. **Public API & Error Architecture:** Granular primitive error types (e.g. `DateError`, `Base64Error`) are marked `pub` but are unnameable by external consumers due to private module containment, alongside asymmetric `validate()` return signatures.
3. **Serde Precision & Data Fidelity:** `Decimal::serialize` converts stored strings to IEEE 754 `f64`, causing silent loss of trailing zeros (`"0.010"` -> `0.01`) and floating-point precision degradation beyond 53 bits.
4. **Documentation & Backlog Drift:** `docs/LLD.md`, `docs/TDD.md`, and `docs/TECH_DESIGN.md` are out of sync with the shipped codebase, still describing the crate as "primitives-only" and omitting the six implemented complex types.

Below are the formalized issues logged for team review and triage.

**Update (2026-09-13):** Every issue was independently re-verified against the actual
code and, where invariants were cited, against hl7.org/fhir/R5 directly, before acting
on it. ISSUE-001, 004, 005, 008, and 010 are fixed. ISSUE-003 had factual errors in its
own citations (see its entry) — corrected and partially fixed. ISSUE-006 is real
behavior but was already a documented, deliberate tradeoff, not an unnoticed bug —
severity corrected, left open. ISSUE-002, 007, and 009 are left open as genuine design
decisions or non-trivial architecture work, not because they were dismissed.

---

## Issue Catalog

| Issue ID | Severity | Category | Title | Status |
| :--- | :--- | :--- | :--- | :--- |
| Issue ID | Severity | Category | Title | Status |
| :--- | :--- | :--- | :--- | :--- |
| **ISSUE-001** | **High** | Stubs | 34 Stubs Omit Inherited `Element` Fields (`id`, `extension`) | **Fixed** |
| **ISSUE-002** | **Medium** | Stubs / Architecture | Quantity Profiles Duplication vs Newtype Wrapper Pattern | Open (design decision, not a bug — see note) |
| **ISSUE-003** | **Medium** | Stubs / Spec | Missing Invariant Documentation in Complex Type Stubs | Partially fixed — see note (this issue's own citations had errors) |
| **ISSUE-004** | **High** | Public API | Granular Primitive Error Types Are Unnameable in Public API | **Fixed** |
| **ISSUE-005** | **Low** | Public API | Inconsistent `validate()` Return Signatures Across Primitives | **Fixed** |
| **ISSUE-006** | ~~High~~ **Low** (see correction) | Serde / Fidelity | `Decimal` Serialization Loses Precision and Trailing Zeros | Open — deliberate, already documented in `decimal/mod.rs` |
| **ISSUE-007** | **Medium** | Ergonomics | Complex Types Lack Builders and `into_parts` Deconstructors | Open (design decision, not a bug — see note) |
| **ISSUE-008** | **Medium** | Documentation | Architectural Docs (`LLD.md`, `TDD.md`, `TECH_DESIGN.md`) Drift | **Fixed** |
| **ISSUE-009** | **Medium** | Architecture | Additive Feature Flags Hazard (`r4` vs `r5`) | Open — real, no strategy decided yet |
| **ISSUE-010** | **Low** | Codebase Hygiene | Dangling Orphan Module `src/r5/mod.rs` | **Fixed** (wired into `lib.rs`) |

**Note on this table**: several entries below were corrected after independent
verification found errors in the original audit — see the per-issue notes, especially
ISSUE-003 (wrong invariant IDs/FHIRPath text for `SampledData`) and ISSUE-006
(overlooked that the behavior it flags is already documented as a deliberate tradeoff,
not an unnoticed defect). Treat this file as reviewed-and-corrected, not as originally
generated.

---

### ISSUE-001: 34 Stubs Omit Inherited `Element` Fields (`id`, `extension`)

- **Severity:** High (Structural)
- **Component:** `src/datatypes/complex/*/mod.rs` (all 34 stub files)
- **Description:**  
  In HL7 FHIR R5, every complex data type inherits from `Element` (or `BackboneType`/`DataType`), which dictates that every element may carry an optional `id` (`string`) and repeated `extension` (`Extension`).  
  All seven implemented complex types (`Extension`, `Period`, `Coding`, `Quantity`, `CodeableConcept`, `Identifier`, `Reference`) correctly include:
  ```rust
  id: Option<FhirString>,
  extension: Vec<Extension>,
  ```
  However, all 34 recently added stubs (e.g. `HumanName`, `Address`, `Annotation`, `Range`, `Ratio`, `Timing`, etc.) omit `id` and `extension` in their `struct` definitions, despite their doc comments stating `"plus id/extension"`.
  Furthermore, nested `BackboneElement` structures (`TimingRepeat`, `DoseAndRate`, `ElementBase`, `CodeFilter`, `DateFilter`, `ValueFilter`, `Sort`) omit `id`, `extension`, and `modifierExtension`.
- **Impact:**  
  Implementing these stubs as currently declared would produce types that cannot hold element IDs, cannot be extended with FHIR extensions, and cannot validate the core `ele-1` invariant.
- **Action Items:**
  1. Add `id: Option<FhirString>` and `extension: Vec<Extension>` to all 34 top-level stub structs.
  2. Add `id`, `extension`, and `modifierExtension: Vec<Extension>` to all nested backbone element stub structs.
- **Resolution (2026-09-13):** Fixed. All 34 top-level stubs now carry `id`/`extension`;
  all 9 nested `BackboneElement` structs (`TimingRepeat`, `DoseAndRate`, `ElementBase`,
  `CodeFilter`, `DateFilter`, `ValueFilter`, `Sort`, `AvailableTime`,
  `NotAvailableTime`) carry `id`/`extension`/`modifier_extension`. Adding the fields
  grew several choice-type enums (`DataRequirementDateValue`, `TimingBounds`,
  `UsageContextValue`) past `clippy::large_enum_variant`'s threshold; the oversized
  variants were boxed. `ElementDefinition` additionally gained `modifier_extension` at
  the top level based on its confirmed `BackboneType` parentage, despite one
  (likely-truncated) fetch claiming no such field is listed — flagged in that module's
  doc for re-verification, consistent with its already-partial status.

---

### ISSUE-002: Quantity Profiles Duplication vs Newtype Wrapper Pattern

- **Severity:** Medium (Architecture / DRY)
- **Component:** `src/datatypes/complex/{age, count, distance, duration, money_quantity, simple_quantity}/mod.rs`
- **Description:**  
  FHIR defines `Age`, `Count`, `Distance`, `Duration`, `MoneyQuantity`, and `SimpleQuantity` as profiles of `Quantity`. They share identical structural fields (`value`, `comparator`, `unit`, `system`, `code`), differentiating only in profile-level constraints (e.g. `SimpleQuantity` disallows `comparator`, `Age` requires non-negative value and UCUM time units).  
  Currently, each of the 6 stubs declares a standalone struct duplicating all 5 fields.
- **Impact:**  
  If implemented as standalone structs, the crate will duplicate ~370 lines of custom visitor, slot accumulator, companion split/merge, and accessor code across 6 files (~2,200 lines of redundant code).
- **Action Items:**
  1. Evaluate designing quantity profiles as newtype wrappers around `Quantity`:
     ```rust
     #[derive(Debug, Clone, PartialEq, Eq)]
     pub struct SimpleQuantity(Quantity);
     ```
  2. Implement profile-specific validating constructors on the newtypes while delegating storage, serialization, and deserialization directly to `Quantity`.
- **Note (2026-09-13):** Left open deliberately, not fixed reflexively. This is a real
  design tradeoff — a newtype wrapper avoids the field duplication but also means the 6
  profile types can't independently evolve field-level docs/derives from `Quantity`,
  and `docs/LLD.md`/`TECH_DESIGN.md` explicitly caution against building abstractions
  ahead of a concrete need. Worth deciding *when* one of these 6 stubs is actually
  picked up for implementation, not now while all 6 are still inert stubs with no
  consumer.

---

### ISSUE-003: Missing Invariant Documentation in Complex Type Stubs

- **Severity:** Medium (Spec Fidelity)
- **Component:** `src/datatypes/complex/{range, ratio, sampled_data, timing, attachment}/mod.rs`
- **Description:**  
  Multiple stubs note "no named invariants found on the summary page" or have unverified invariant notes, whereas FHIR R5 defines normative error-severity invariants:
  - `Range`: Invariant `rng-2` (`low.value.empty() or high.value.empty() or (low <= high)`).
  - `Ratio`: Invariant `rat-1` (`(numerator.empty() xor denominator.exists()) and (numerator.exists() or extension.exists())`).
  - `SampledData`: Invariant `smd-1` (`offsets.empty() or data.empty()`).
  - `Timing`: Invariants `tim-1` through `tim-9` (e.g. `durationMax` requires `duration`, `frequencyMax` requires `frequency`).
  - `Attachment`: Gating `size: Option<Primitive<Integer64>>` behind `r5` leaves `Attachment` with no `size` field under `r4`, whereas R4 defines `size: unsignedInt`.
- **Action Items:**
  1. Update module documentation in each stub to cite the normative invariant keys and FHIRPath definitions from `datatypes-definitions.html`.
  2. Record which invariants can be checked purely locally vs which require external units or calendar boundary expansion.
- **Correction (2026-09-13) — this issue's own citations were re-verified against
  hl7.org/fhir/R5/datatypes-definitions.html and had errors:**
  - `SampledData`'s invariant id is **`sdd-1`**, not `smd-1`, and its actual rule is
    "A SampledData SHALL have either an interval and offsets but not both"
    (`interval.exists().not() xor offsets.exists().not()`) — not the
    "`offsets.empty() or data.empty()`" text given above.
  - `Ratio`'s `rat-1` does exist, but the FHIRPath quoted above doesn't match the spec
    text either; the confirmed expression is `(numerator.exists() and
    denominator.exists()) or (numerator.empty() and denominator.empty() and
    extension.exists())`.
  - `Range`'s `rng-2` is confirmed accurate as stated (modulo the real expression using
    `lowBoundary()`/`highBoundary()`, not a plain `<=`).
  - `Timing`'s `tim-1`..`tim-9` and `Attachment`'s `att-1` were not independently
    re-verified here; treat those two as still-unconfirmed pending a direct fetch, same
    status as before this correction.
  - **Fixed in the stub docs** (`range`, `ratio`, `sampled_data`, `attachment`
    modules) as part of the ISSUE-001 pass — each now cites its confirmed invariant
    (or confirmed absence of one) with the corrected id/FHIRPath/severity.
    `Timing`'s stub still says "not yet checked," accurately.

---

### ISSUE-004: Granular Primitive Error Types Are Unnameable in Public API

- **Severity:** High (Public API Ergonomics)
- **Component:** `src/types/mod.rs` and all `src/types/*/mod.rs`
- **Description:**  
  Each primitive type exposes a public validation function, such as:
  ```rust
  pub fn validate(value: &str) -> Result<(), DateError>;
  pub fn validate(value: &str) -> Result<(), Base64Error>;
  pub fn validate(value: &str) -> Result<(), DecimalError>;
  ```
  While `DateError`, `Base64Error`, etc., are declared `pub enum`, their enclosing modules (`mod date;`, `mod base64;`) in `src/types/mod.rs` are private, and the error types are not re-exported.
- **Impact:**  
  Downstream users calling `Date::validate(...)` receive an error value whose type cannot be named, imported, or pattern-matched outside the `fhir-core` crate.
- **Action Items:**
  1. Either re-export each error type in `src/types/` (e.g. `pub use date::DateError;`) or re-export them under a dedicated error module (`fhir_core::errors::primitives::*`).
  2. Alternatively, align with `docs/TECH_DESIGN.md` §2 and have all `validate()` methods return `Result<(), TypeError>`.
- **Resolution (2026-09-13):** Fixed via option 1 — all 18 granular error enums
  (`Base64Error`, `CanonicalError`, `CodeError`, `DateError`, `DateTimeError`,
  `DecimalError`, `IdError`, `InstantError`, `IntegerError`, `Integer64Error`,
  `MarkdownError`, `OidError`, `PositiveIntError`, `StringError`, `TimeError`,
  `UnsignedIntError`, `UriError`, `UrlError`, `UuidError`) are now re-exported from
  `src/types/mod.rs`, each behind the same feature gate as its owning type.

---

### ISSUE-005: Inconsistent `validate()` Return Signatures Across Primitives

- **Severity:** Low (API Consistency)
- **Component:** `src/types/{integer, integer64, positive_int, unsigned_int}/mod.rs`
- **Description:**  
  Validation functions across primitives have divergent signatures:
  - String-backed primitives return `Result<(), Error>` (e.g. `Date::validate`, `Uri::validate`).
  - Integer primitives return `Result<NativeType, Error>` (e.g. `Integer::validate(&str) -> Result<i32, IntegerError>`, `PositiveInt::validate(&str) -> Result<u32, PositiveIntError>`).
- **Impact:**  
  Violates API symmetry and complicates generic validation wrappers.
- **Action Items:**
  1. Standardize all `validate` signatures to `validate(&str) -> Result<(), E>`.
  2. Provide dedicated parsing methods (e.g. `parse(&str) -> Result<T, E>` or standard `FromStr`) for obtaining the parsed primitive.
- **Resolution (2026-09-13):** Fixed via both action items — `Integer`, `Integer64`,
  `PositiveInt`, `UnsignedInt` each now have `validate(&str) -> Result<(), E>` (matching
  every other primitive) plus a new `parse(&str) -> Result<NativeType, E>` for callers
  that want the parsed value; `TryFrom` now calls `parse` internally. All existing
  tests/doctests that relied on `validate`'s old return value were moved to test
  `parse` instead, and new tests were added for `validate`'s `Result<(), E>` shape.

---

### ISSUE-006: `Decimal` Serialization Loses Precision and Trailing Zeros

- **Severity:** High (Data Fidelity)
- **Component:** `src/types/decimal/mod.rs`
- **Description:**  
  The FHIR specification requires `decimal` to preserve precision (including trailing zeros like `"0.010"` vs `"0.01"`). While `Decimal` correctly stores an internal `String`, its `Serialize` implementation converts non-integers to `f64`:
  ```rust
  serializer.serialize_f64(self.as_f64())
  ```
- **Impact:**  
  1. `serde_json` serializes `0.010` as `0.01`, losing trailing-zero precision upon round-trip.
  2. Decimals with more than 15-17 significant digits (FHIR allows up to 18 integer digits + 17 fractional digits = 35 digits) suffer silent floating-point precision corruption.
- **Action Items:**
  1. Implement custom serializer support for JSON raw values (e.g. via `serde_json::value::RawValue` when `serde_json` is available or feature-flagged).
  2. Document the exact limits and precision behavior prominently in `docs/TECH_DESIGN.md` and module docs.
- **Correction (2026-09-13) — severity downgraded, status left Open:** the *behavior*
  described is accurate, but this is not an unnoticed defect. `src/types/decimal/mod.rs`
  already documents this exact tradeoff in detail (search "accepted limitation, not a
  bug"): serde's data model has no arbitrary-precision numeric primitive, the lossy
  step already happens on the *deserialize* side (a JSON token like `4.5600` becomes an
  `f64` bit pattern before any of this crate's code runs), and the module doc gives the
  full rationale for why `Decimal::as_f64()` exists as an explicit opt-in rather than
  fixing this implicitly. Action item 1 (`RawValue`) is worth evaluating for real, but
  as a considered enhancement to a documented tradeoff — not a High-severity bug fix.
  Re-flagging this as freshly-discovered would have caused duplicate rediscovery work;
  left at Open/Low here specifically so it doesn't get triaged as urgent.

---

### ISSUE-007: Complex Types Lack Builders and `into_parts` Deconstructors

- **Severity:** Medium (Ergonomics)
- **Component:** `src/datatypes/complex/*/mod.rs`
- **Description:**  
  All implemented complex types have private fields and require all-argument constructors. For example:
  - `Identifier::new` requires 8 positional arguments (`use`, `type`, `system`, `value`, `period`, `assigner`, `id`, `extension`).
  - `Quantity::new` requires 7 positional arguments.
  Constructing a simple identifier with only `system` and `value` requires passing six `None`/empty arguments.
  Furthermore, there are no `into_parts(self)` methods, making it impossible to unpack fields without cloning.
- **Action Items:**
  1. Add typed Builder structs for complex types (e.g. `IdentifierBuilder`, `QuantityBuilder`).
  2. Add `into_parts(self)` methods alongside accessor methods.
- **Note (2026-09-13):** Left open deliberately. `Extension::new` (4 args) already
  established the positional-constructor convention before this audit; `Identifier`'s 8
  args are a real ergonomics cost but a builder is exactly the kind of abstraction
  `docs/LLD.md`/`TECH_DESIGN.md` say not to add ahead of a caller who's actually hitting
  the pain. Revisit if/when a real consumer finds the positional constructors
  unworkable, not preemptively.

---

### ISSUE-008: Architectural Docs (`LLD.md`, `TDD.md`, `TECH_DESIGN.md`) Drift

- **Severity:** Medium (Documentation Integrity)
- **Component:** `docs/LLD.md`, `docs/TDD.md`, `docs/TECH_DESIGN.md`
- **Description:**  
  Substantial documentation drift was detected:
  - `docs/LLD.md`: §1 and §4 still state that complex types are deferred and only 20 primitives are implemented. §3 table lists R4 `base64Binary` regex. §6 directory tree omits implemented complex types and stubs.
  - `docs/TDD.md`: §4 omits test coverage documentation for `Period`, `Coding`, `Quantity`, `CodeableConcept`, `Identifier`, and `Reference`. §3 lists `base64Binary` internal whitespace as a happy-path test.
  - `docs/TECH_DESIGN.md`: §3 states complex types is "currently just Extension". §7 states no `r4` support is planned, despite `Cargo.toml` featuring `r4 = []`.
- **Action Items:**
  1. Update `docs/LLD.md` to document the 7 implemented complex types and 34 stubs.
  2. Update `docs/TDD.md` to reflect test coverage for the 7 complex types.
  3. Update `docs/TECH_DESIGN.md` to reflect current architecture and feature flags.
- **Resolution (2026-09-13):** Fixed — all three docs updated. Also fixed in the same
  pass: `LLD.md` §3's `base64Binary` regex row (was describing the R4 grammar, not
  what's actually implemented — folded into the existing `docs/BACKLOG.md`
  `base64Binary` entry, which tracked this independently) and `TDD.md` §3's matching
  "with internal whitespace" happy-path vector (R5 forbids whitespace; that vector was
  never actually exercised against R5's `Base64Binary`).

---

### ISSUE-009: Additive Feature Flags Hazard (`r4` vs `r5`)

- **Severity:** Medium (Cargo Architecture)
- **Component:** `Cargo.toml`, `src/lib.rs`, `src/types/mod.rs`
- **Description:**  
  `Cargo.toml` defines `r4 = []` and `r5 = []` as separate features. Cargo features are additive across dependency graphs. If one crate in a dependency graph requests `r4` and another requests `r5`, both features are enabled simultaneously.  
  Currently, types differ between R4 and R5:
  - `base64Binary`: R4 permits internal whitespace; R5 forbids it.
  - `Attachment.size`: R4 is `unsignedInt`; R5 is `integer64`.
  - `integer64`: present in R5, absent in R4.
- **Impact:**  
  With both features active, conditional compilation like `#[cfg(feature = "r5")]` overrides R4 types, making dual-version compilation semantically invalid.
- **Action Items:**
  1. Decide on a multi-spec strategy: either versioned namespaces (`fhir_core::r4::*` vs `fhir_core::r5::*`) or mutual exclusivity assertions / compiler errors if both are selected without namespacing.
- **Status (2026-09-13):** Still open — this is a real architecture decision, not a
  quick fix, and `docs/BACKLOG.md`'s `base64Binary` entry already tracks the same
  problem from the single-type angle. `TECH_DESIGN.md` §7 now names this issue
  explicitly instead of denying `r4` support exists.

---

### ISSUE-010: Dangling Orphan Module `src/r5/mod.rs`

- **Severity:** Low (Hygiene)
- **Component:** `src/r5/mod.rs`
- **Description:**  
  The file `src/r5/mod.rs` exists with doc text `"//! FHIR Release 5 (R5) data models and resource structures."`, but is never declared with `pub mod r5;` in `src/lib.rs`.
- **Action Items:**
  1. Either wire `#[cfg(feature = "r5")] pub mod r5;` into `src/lib.rs` or delete `src/r5/` until resource modelling work begins.
- **Resolution (2026-09-13):** Fixed — wired in (option 1), matching
  `docs/TECH_DESIGN.md`'s stated intent for this module to eventually hold R5 resource
  models.

---

## Next Steps & Recommended Implementation Roadmap

~~1. **Triage & Priority Alignment:** Review ISSUES 001, 004, and 006 first, as they impact public API stability and serialization correctness.~~
~~2. **Docs Synchronization:** Resolve ISSUE-008 by bringing `LLD.md`, `TDD.md`, and `TECH_DESIGN.md` up to date with git HEAD.~~
~~3. **Stubs Architecture:** Prioritize ISSUE-001 (adding `id`/`extension` to stubs)~~ and ISSUE-002 (Quantity profiles newtype decision) before writing implementations for any of the 34 stubs.

**Status (2026-09-13):** Steps 1 and 2 done — ISSUE-001, 004, 005, 008, and 010 fixed;
ISSUE-006's severity corrected (documented tradeoff, not a fresh bug); ISSUE-003's own
citations corrected and the confirmed invariants folded into the relevant stub docs.
Remaining open items are genuine design decisions, not oversights: ISSUE-002 (newtype
vs. duplicated-fields for the six `Quantity` profiles) and ISSUE-007 (builders) should
be decided when a stub is actually picked up for implementation, not preemptively;
ISSUE-009 (R4/R5 additive-feature-flags strategy) is real, non-trivial architecture
work already tracked in `docs/BACKLOG.md`'s `base64Binary` entry.
