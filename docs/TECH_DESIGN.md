# Technical Design Document: `fhir-core`

**Document Version:** 0.1.0
**Status:** Draft — describes shipped primitives plus the immediate next step
**Standard:** HL7 FHIR Release 5 (R5) Normative
**MSRV:** Rust 1.87 (Edition 2024)
**Safety Mandate:** `#![forbid(unsafe_code)]`, zero non-serde runtime dependencies.

---

## 1. System Overview & Context

`fhir-core` is a Rust library providing FHIR R5 primitive types with spec-conformant
validation. It is meant as a foundation for higher-level FHIR software (servers,
clients, ETL, FHIRPath), but does not itself model resources yet.

### 1.1 Non-Goals

- **No Resource Bloat in Core**: this crate does not bundle generated FHIR resource
  definitions (`Patient`, `Observation`, ...); those build on top of it.
- **No Heavy Runtime Dependencies**: no regex crates, bignum libraries, or async
  runtimes. Validation is hand-rolled, zero-allocation, single-pass character scanning.
- **No speculative architecture**: traits, wrapper types, and complex data types are
  not designed until a concrete consumer needs them (see LLD §4).

---

## 2. Architectural Drivers & Quality Attributes (NFRs)

| Attribute | Target Metric | Architectural Strategy |
| :--- | :--- | :--- |
| **Safety** | `#![forbid(unsafe_code)]` | 100% safe Rust, no `unsafe` blocks. |
| **Dependencies** | 0 non-serde dependencies | Only optional `serde` dependency in production builds. |
| **Memory Efficiency** | Minimal heap allocation | `&str` slices for string-backed primitives; native primitives (`i32`, `i64`, `bool`, `u32`) stored directly. |
| **Validation Fidelity** | 100% HL7 R5 conformance | Regexes/ranges verified against the spec page per type, not recalled. |
| **Errors** | Structured, typed | `TypeError::InvalidValue { type, value, error }` for single-field grammar, `ConstraintError::InvariantViolated { key, description }` for multi-field invariants (e.g. `ext-1`) — all-`String`/`&'static str` fields, no byte-index/char diagnostics yet. Richer variants are added only if a real caller needs them. |
| **Modern Rust Baseline** | Edition 2024, MSRV 1.87 | — |

---

## 3. Subsystem Architecture

Three subsystems so far:
- **Primitives Engine** (`src/types/`) — 20 of the 21 normative FHIR R5 primitive types
  (`xhtml` not yet implemented), each a standalone struct with construction-time
  validation. An `r4` feature flag exists in `Cargo.toml`, but only `base64Binary`'s
  R4-vs-R5 grammar difference has actually been audited so far — see `docs/BACKLOG.md`.
- **Element Wrapper** (`src/datatypes/primitive/`) — `Primitive<T>`, the
  `{value, id, extension}` shape every primitive-valued property structurally is, plus
  the `_propertyName` companion serde split/merge helpers hand-rolled containers use.
- **Complex Types** (`src/datatypes/complex/`) — seven fully implemented: `Extension`,
  `Period`, `Coding`, `Quantity`, `CodeableConcept`, `Identifier`, `Reference`. Each is
  a plain struct (no traits) with a validating constructor for its error-severity
  invariants, `id`/`extension` fields, accessors, and hand-rolled serde; each is wired
  into `Extension` as an `ExtensionValue` variant. `Identifier` and `Reference` are
  mutually recursive (each embeds the other, boxed). A further 34 types (general-purpose,
  metadata, and special-purpose) exist as doc-only stubs — real fields, spec-cited
  module docs, but no construction/validation/serde/wiring — see `docs/BACKLOG.md`
  Roadmap for the full list and build order.

`src/errors/` provides the shared error types both return (`TypeError` for single-field
grammar, `ConstraintError` for multi-field invariants like `ext-1`/`ele-1`). Each
primitive's granular error enum (`DateError`, `Base64Error`, etc.) is also re-exported
from `src/types/` so external callers can name and match on it, not just the crate-wide
`TypeError` its `TryFrom` impls map into.

Everything past the seven implemented complex types (`Base`/`Element`/`Resource` trait
hierarchy, the remaining 34 complex types, choice-type enum growth) is deferred — see
LLD.md §4 for why and how it gets designed when actually needed.

---

## 4. Memory Model & Layout

### 4.1 Native Scalar Types
- `Boolean`: `struct Boolean(bool)`.
- `Integer`: `struct Integer(i32)`.
- `Integer64`: `struct Integer64(i64)`.
- `PositiveInt`, `UnsignedInt`: `struct _(u32)`.

### 4.2 String-Backed Primitives
- `Decimal`: `struct Decimal(String)` — stores the validated representation directly so
  trailing zeros (`0.010` vs `0.01`) are never lost.
- `FhirString`: `struct FhirString(String)`, max 1,048,576 chars.
- `Id`: `struct Id(String)`, max 64 chars.
- Remaining string-backed primitives follow the same shape once implemented.

---

## 5. Serde & Interoperability

### 5.1 The `Integer64` Special Case
HL7 FHIR R5 requires `integer64` to serialize as a JSON string
(`"9223372036854775807"`), not a JSON number, because JS runtimes parse JSON numbers as
64-bit floats and lose precision above $2^{53}-1$.
- `Integer64::serialize` calls `serializer.serialize_str(&self.0.to_string())`.
- `Integer64::deserialize` accepts a JSON string or integer, validating
  $[-2^{63}, 2^{63}-1]$.

### 5.2 The `_propertyName` Companion Protocol
Any primitive-valued property may carry its own `id`/`extension` via a `_`-prefixed
sibling key (`"gender": "male", "_gender": {"id": "g1"}`), present only as needed
(bare key alone, `_`-key alone for the data-absent-reason case, or both). `Primitive<T>`
has no `Serialize`/`Deserialize` of its own since it isn't one JSON value in general —
`src/datatypes/primitive/mod.rs`'s `serialize_primitive_entry`/`merge_primitive_entry`
do the split/merge, called by each hand-rolled container (`Extension`'s impl today).
See LLD.md §4.2.

---

## 6. Threat Modeling

### 6.1 ReDoS Elimination
No external regex engine is used anywhere in the crate — every validator is a
single-pass $O(N)$ scanner with a fixed length cap, so there is no backtracking surface.

### 6.2 Memory Exhaustion
- `FhirString`/`Markdown` capped at 1,048,576 characters.
- `Id` capped at 64 characters.
- Numeric inputs parse through bounds-checked Rust parsing.

---

## 7. Versioning

Feature flags `r4` and `r5` gate primitives/complex types (`r5` default). Both are
Cargo-additive: a dependency graph that pulls in both is a real possibility, not a
theoretical one, and `#[cfg(feature = "r5")]` cannot itself distinguish "R5 requested
instead of R4" from "R5 requested alongside R4" — see `docs/BACKLOG.md`
(`base64Binary` entry) and `REVIEW_ISSUES.md` ISSUE-009 for the versioning-strategy
decision this still needs. Only `base64Binary`'s R4 grammar and `Attachment.size`'s
R4 type (`unsignedInt`, vs R5's `integer64`) have been confirmed to differ across
versions; the rest of the primitive/type set is unaudited for R4 fidelity. `r4` is a
real near-term target, not a placeholder, but is not yet usable as one.

---

## 8. CI Verification

Every commit to `develop` must pass:
```bash
make check
```
Which runs:
1. `cargo fmt --all -- --check`
2. `cargo clippy --all-features --all-targets -- -D warnings`
3. `cargo test --all-features --verbose`
4. `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps --all-features`
