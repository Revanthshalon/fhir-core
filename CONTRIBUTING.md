# Contributing to `fhir-core`

Thank you for your interest in contributing to `fhir-core`! This project provides strongly typed, specification-compliant Rust models, primitives, and validation logic for Fast Healthcare Interoperability Resources ([FHIR](https://hl7.org/fhir/)).

To maintain high data integrity and medical standard compliance, all contributions must strictly adhere to the guidelines below.

---

## Table of Contents

- [Core Principles](#core-principles)
- [Development Setup](#development-setup)
- [Git Workflow & Branching](#git-workflow--branching)
- [Implementing FHIR Primitives](#implementing-fhir-primitives)
- [Local Verification & CI Requirements](#local-verification--ci-requirements)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Code Style & Best Practices](#code-style--best-practices)

---

## Core Principles

1. **Strict Official FHIR Compliance**: Never rely on memory or informal conventions. Every primitive and data model must mirror the latest official specification from [HL7 FHIR Release 5 (R5)](https://hl7.org/fhir/R5/).
2. **Robust Invariant Enforcement**: Primitives must validate boundary conditions, character sets, regex patterns, whitespace rules, and sign handling as dictated by the FHIR specification.
3. **Zero Compiler Warnings**: All code must compile cleanly under `#![deny(warnings)]` with Clippy and Rustdoc.
4. **Complete Documentation**: Every public type, method, constant, and module must be thoroughly documented with doc comments and runnable examples.
5. **Exhaustive Testing**: Code coverage must remain high. Test happy paths, boundary conditions, malformed inputs, whitespace wrapping, unicode handling, and Serde JSON roundtrips.

---

## Development Setup

### Prerequisites

- **Rust**: Version `1.87` or higher (Edition 2024).
- **Toolchain Components**: `clippy` and `rustfmt`.
- **Cargo Tarpaulin** (optional, for local coverage):

```bash
rustup update stable
rustup component add clippy rustfmt
cargo install cargo-tarpaulin
```

### Building & Running Tests

```bash
# Clone your fork
git clone https://github.com/<your-username>/fhir-core.git
cd fhir-core

# Build the project
cargo build --all-features

# Run all tests and doctests
cargo test --all-features --verbose
```

---

## Git Workflow & Branching

### Branching Strategy

- **`develop`**: The primary integration branch. **All feature branches and pull requests must branch from and target `develop`**.
- **`main`**: Production releases. Direct pushes and external PRs to `main` are restricted.

### Branch Protection on `develop`

The `develop` branch is protected:
- **Direct pushes are disabled**: All changes must arrive through a Pull Request.
- **Review Requirement**: External pull requests require **at least 1 approving review** from a repository maintainer before merging.
- **Passing Pipeline Checks**: All GitHub Actions CI checks (`Lint & Test`, `Code Coverage`) must pass in strict mode.

### Branch Naming Convention

Name your branches with an appropriate prefix:
- `feat/<feature-name>` (e.g., `feat/integer64-primitive`)
- `fix/<bug-description>` (e.g., `fix/integer-signed-zero-validation`)
- `docs/<description>` (e.g., `docs/contributing-guide`)
- `test/<description>` (e.g., `test/base64-edge-cases`)
- `refactor/<description>` (e.g., `refactor/type-error-display`)

---

## Implementing FHIR Primitives

When adding or modifying a primitive in `src/primitives/<type_name>/`:

1. **Directory Structure**:
   ```text
   src/primitives/<type_name>/
   ├── mod.rs    # Core type definition, validation, traits, doc comments
   └── test.rs   # Unit tests, edge cases, serde roundtrips
   ```

2. **Module Re-export**:
   Re-export the type in [`src/primitives/mod.rs`](src/primitives/mod.rs) under the relevant feature flag (e.g., `#[cfg(feature = "r5")]`).

3. **Pattern & Traits**:
   - Wrap the native primitive (e.g., `bool`, `String`, `i32`).
   - Implement `new(val)` (infallible if all values are valid) and `new_unchecked(val)`.
   - Implement `validate(&str)` returning a granular error enum.
   - Implement standard conversion traits:
     - `TryFrom<&str, Error = TypeError>`
     - `TryFrom<String, Error = TypeError>`
     - `std::str::FromStr<Err = TypeError>`
     - `From<T>` and `From<Primitive> for T`
     - `std::fmt::Display`
   - Implement Serde (`#[derive(Serialize, Deserialize)]` with `#[serde(transparent)]` or explicit serializers).

4. **Error Handling**:
   Define an error enum (e.g., `<Type>Error`) implementing `Display` and `std::error::Error`, and map errors to [`TypeError::InvalidValue`](src/errors/type.rs).

---

## Local Verification & CI Requirements

Before pushing your changes and opening a pull request, run the exact suite executed by CI:

```bash
# 1. Format check
cargo fmt --all -- --check

# 2. Clippy linting (warnings are treated as errors)
cargo clippy --all-features --all-targets -- -D warnings

# 3. Unit and integration tests
cargo test --all-features --verbose

# 4. Documentation check (missing docs are treated as errors)
RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps --all-features

# 5. Test coverage (optional but recommended)
cargo tarpaulin --all-features --engine llvm --skip-clean
```

---

## Submitting a Pull Request

1. **Fork and branch**: Create a feature branch from latest `develop`.
2. **Commit clearly**: Use concise, conventional commit messages (e.g., `feat: implement integer primitive for R5`).
3. **Open PR against `develop`**: Navigate to the repository on GitHub and open a pull request targeting `develop`.
4. **Fill out the Pull Request Template**: Complete all sections of the template, including links to the official FHIR specification and the quality checklist.
5. **Address review feedback**: Maintainers will review your PR and provide feedback. Once the pipeline checks pass and an approval is given, your PR will be merged.

---

## Code Style & Best Practices

- **Avoid Unsafe Code**: The crate should remain `#![forbid(unsafe_code)]` unless an exception is strictly justified and approved.
- **Explicit Domain Types**: Do not implement `Deref` or `DerefMut` on primitive wrappers if it bypasses domain-level validation. Provide explicit accessors like `as_str()`, `as_i32()`, or `into_inner()`.
- **Zero Allocations on Read**: Validation methods should accept string slices (`&str`) and avoid heap allocations where feasible.
- **Preserve Documentation Integrity**: Update documentation comments when modifying behavior, and keep examples copy-paste runnable in doctests.
