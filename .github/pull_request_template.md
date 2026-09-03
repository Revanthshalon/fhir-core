## Description

<!-- Provide a concise summary of the changes introduced by this pull request. -->

## FHIR Specification Reference

<!-- Link to the official HL7 FHIR specification (e.g., FHIR R5) documenting the requirements, regex, and invariants for this type or feature. -->
- **Specification URL**: 
- **Version**: FHIR Release 5 (R5)
- **Key Invariants / Constraints**: 

## Type of Change

<!-- Mark the appropriate item with an [x]. -->
- [ ] `feat`: New primitive type, data model, or feature
- [ ] `fix`: Bug fix or specification compliance correction
- [ ] `docs`: Documentation updates or additions
- [ ] `refactor`: Code refactoring without changing functionality
- [ ] `test`: New or updated tests
- [ ] `ci`: Workflow, pipeline, or tooling adjustments

## Related Issue

<!-- If applicable, link to the related issue (e.g., Closes #123, Fixes #456). -->

## Quality & Compliance Checklist

Before requesting a review, verify the following:

- [ ] **Target Branch**: This PR targets the `develop` branch (not `main`).
- [ ] **Formatting**: Code is formatted via `cargo fmt --all -- --check`.
- [ ] **Clippy**: Code passes all linter checks with zero warnings via `cargo clippy --all-features --all-targets -- -D warnings`.
- [ ] **Tests**: All unit and doc tests pass via `cargo test --all-features`.
- [ ] **Documentation**: All public types, functions, constants, and modules are documented with zero missing docs (`RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps --all-features`).
- [ ] **Test Coverage**: Added comprehensive test coverage including:
  - [ ] Happy path / valid values
  - [ ] Negative validation / malformed inputs
  - [ ] Boundary limits (e.g. min/max ranges, string length limits)
  - [ ] Whitespace, leading zero, and format restrictions
  - [ ] Serde JSON serialization & deserialization roundtrips
- [ ] **Specification Fidelity**: Changes have been verified against the official HL7 FHIR specification.
