# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`fhir-core` is a Rust crate providing strongly typed, specification-compliant primitives, data types,
and validation logic for FHIR (Fast Healthcare Interoperability Resources), R5. Zero non-serde
dependencies; `#![forbid(unsafe_code)]`.

## Commands

```bash
# Format check (CI runs this, not `cargo fmt` — check only, no write)
cargo fmt --all -- --check

# Lint (warnings are errors)
cargo clippy --all-features --all-targets -- -D warnings

# Tests (includes doctests)
cargo test --all-features --verbose

# Run a single test
cargo test --all-features <test_name>

# Docs (missing docs are errors — every public item must be documented)
RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps --all-features

# Coverage (optional locally, required to stay high)
cargo tarpaulin --all-features --engine llvm --skip-clean

# All of the above except coverage
make check
```

Run the full `check` suite (fmt, clippy, test, doc) before considering any change done — this is
exactly what CI enforces on every PR into `develop`.

## Architecture

- `src/errors/` — crate-wide error types. `FhirCoreError` (in `mod.rs`) is the top-level enum that
  wraps subsystem errors; currently only variant is `Type(TypeError)`. `type.rs` defines `TypeError`,
  the granular error used by every primitive's validation failure (`InvalidValue { type, value, error }`).
  `constraints.rs` is a placeholder for future FHIR invariant/constraint evaluation errors.
- `src/primitives/` — each FHIR primitive type lives in its own submodule
  (`src/primitives/<type_name>/mod.rs` + `test.rs`), gated behind `#[cfg(feature = "r5")]` and
  re-exported from `src/primitives/mod.rs`. Existing primitives: `base64` (`Base64Binary`),
  `boolean` (`Boolean`), `decimal` (`Decimal`), `integer` (`Integer`), `integer64` (`Integer64`),
  `string` (`FhirString`).
- `src/r5/` — currently an empty placeholder module for future FHIR R5 resource/data models built
  on top of the primitives.
- Feature flags: `r5` (default) gates all R5 primitives/models; `serde` (default) gates
  serialization support. Both are additive — code should compile with either disabled.

### Primitive type pattern

Every primitive follows the same shape (see `src/primitives/decimal/mod.rs` for the most fully
worked example, including extensive module-doc rationale for its non-obvious design choices):

- Wraps the native Rust type (`bool`, `String`, `i32`, ...) — or, when spec semantics can't be
  captured by a numeric type (e.g. `Decimal` needing to preserve trailing-zero precision), wraps
  the validated string itself.
- `new(val)` — infallible constructor when all native values are valid (e.g. `Boolean`); otherwise
  a fallible constructor.
- `new_unchecked(val)` — skips validation; caller is responsible for the invariant afterward (some
  accessors may panic if it's violated).
- `validate(&str) -> Result<(), TypeError>` (or equivalent) performing the FHIR-spec grammar checks.
- Conversion traits: `TryFrom<&str, Error = TypeError>`, `TryFrom<String, Error = TypeError>`,
  `FromStr<Err = TypeError>`, `Display`, plus `From`/`Into` conversions to/from the wrapped native type.
- No `Deref`/`DerefMut` — use explicit accessors (`as_str()`, `as_bool()`, `as_f64()`,
  `into_inner()`, ...) so validation isn't bypassable.
- Serde support via `#[derive(Serialize, Deserialize)]` (transparent where possible) or explicit
  `Serializer`/`Deserializer` impls when the JSON encoding differs from the Rust representation
  (e.g. `Decimal` serializes as a JSON number, `Integer64` as a JSON string per spec).
- Errors surface through `TypeError::InvalidValue`, not a bespoke per-type error escaping the crate
  boundary — a type-specific error enum (implementing `Display`/`Error`) can exist internally and
  map into `TypeError`.

When adding a new primitive: create `src/primitives/<type_name>/{mod.rs,test.rs}`, gate it behind
`#[cfg(feature = "r5")]`, and re-export it from `src/primitives/mod.rs`.

## Conventions specific to this repo

- **Spec-driven, not memory-driven**: every primitive's grammar (regex, digit/length limits,
  encoding) must be verified against the official HL7 FHIR R5 spec
  (hl7.org/fhir/R5/datatypes.html), not implemented from general knowledge. Module docs cite the
  spec section and regex directly (see `decimal/mod.rs` for the expected level of detail).
- **Document *why*, not just *what***, especially for non-obvious spec-compliance tradeoffs (e.g.
  why `Decimal` doesn't implement `Ord`, why JSON round-tripping is lossy for some values). This is
  a documented crate convention, not incidental style.
- Tests live in a sibling `test.rs` per module (not inline `#[cfg(test)] mod tests` at the bottom),
  and must cover: happy path, malformed/negative input, boundary limits, whitespace/leading-zero/
  format edge cases, and serde JSON roundtrips.
- Target branch for all PRs is `develop`, not `main`. Branch naming: `feat/`, `fix/`, `docs/`,
  `test/`, `refactor/` prefixes.
