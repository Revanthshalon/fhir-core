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

Two subsystems so far:
- **Primitives Engine** (`src/types/`) — 20 normative FHIR R5 primitive types, each a
  standalone struct with construction-time validation.
- **Complex Types** (`src/datatypes/complex/`) — currently just `Extension`, a plain
  struct (no traits) enforcing the `ext-1` multi-field invariant.

`src/errors/` provides the shared error types both return (`TypeError` for single-field
grammar, `ConstraintError` for multi-field invariants like `ext-1`).

Everything past that (Element/Resource trait hierarchy, `Primitive<T>` wrapper,
`_propertyName` companion serde handling, the remaining complex types, choice-type enum
growth) is deferred — see LLD.md §4 for why and how it gets designed when actually
needed.

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

The `_propertyName` companion-field protocol (for `id`/`extension` on primitive JSON
fields) is deferred along with `Primitive<T>` — see LLD.md §4.

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

Feature flag `r5` (default) gates all primitives. No `r4`/`r4b` support exists or is
planned until there's a concrete reason to add it.

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
