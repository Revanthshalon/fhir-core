# Test-Driven Development (TDD) Plan: `fhir-core`

**Version:** 0.1.0
**Target Standard:** HL7 FHIR Release 5 (R5)
**CI Gate:** `make check` (zero warnings, 100% tests passing, zero clippy lints, zero missing docs).

---

## 1. TDD Philosophy

1. **Test-First**: for every type, tests in `test.rs` are authored before the
   implementation.
2. **Zero Unchecked Assumptions**: grammar, regex patterns, and length limits are
   cross-referenced directly with hl7.org/fhir/R5/datatypes.html, not recalled from
   memory.
3. **Isolated Sibling Test Modules**: unit tests live in a sibling `test.rs` within the
   type's module directory (e.g. `src/types/date/test.rs`).
4. **Error assertions**: negative tests assert the exact `TypeError::InvalidValue`
   returned (its `value`/`error` message), not just `is_err()` — the crate does not
   currently have per-type error variants with byte-index/char diagnostics, so that's
   the granularity available today.

---

## 2. Test Layers (apply to what's built)

1. **Grammar & boundary** — raw string parsing, regex conformance, length caps,
   whitespace handling; positive, negative, and adjacent-boundary vectors.
2. **Conversion traits** — `TryFrom<&str>`, `TryFrom<String>`, `FromStr`, `Display`,
   `into_inner()`, `new_unchecked()` (verifies it stores the value verbatim, skipping
   validation).
3. **Serde round-trip** — `to_string`/`from_str` and correct JSON token shape (e.g.
   `integer64` as a JSON string, `decimal` as a JSON number).

Layer for invariant checks now covers `ele-1` (`Primitive<T>`) and `ext-1` (`Extension`),
including the `_propertyName` companion protocol (§4.1, §4.2). Remaining invariants
(`per-1`, ...) are specified alongside whichever complex type first needs them (see
LLD.md §4), not ahead of time.

---

## 3. Primitives Test Matrix

| Primitive | Status | Happy Path Vectors | Negative / Edge Vectors |
| :--- | :--- | :--- | :--- |
| `base64Binary` | Complete (R5 only) | `"TWFu"`, `"AAAA"`, empty string | Internal whitespace (R5 forbids it — R4 permits it and is not yet implemented), odd lengths, illegal chars (`@`, `!`), invalid padding (`===`) |
| `boolean` | Complete | `true`, `false`, `"true"`, `"false"` | `"True"`, `"FALSE"`, `"yes"`, `"1"`, `""` |
| `decimal` | Complete | `"0"`, `"-0"`, `"0.010"`, `"42.5"`, `"1e3"`, `"-1.2E-3"` | `"+42"`, `".5"`, `"5."`, `"0123"`, non-numeric, finite check |
| `id` | Complete | `"a"`, `"patient-1234"`, `"Observation.1"`, 64-char string | `""`, 65-char string, spaces, `_`, `/`, `@`, unicode |
| `integer` | Complete | `0`, `42`, `-100`, `"+42"`, `"0"`, `i32::MIN`, `i32::MAX` | `"-0"`, `"+0"`, `"01"`, `i32::MAX + 1`, `42.5`, letters |
| `integer64` | Complete | `0`, `42`, `i64::MIN`, `i64::MAX`, serialized as `"42"` | `"-0"`, `"+0"`, `"01"`, `i64::MAX + 1`, raw JSON numbers |
| `string` | Complete | `"Hello"`, unicode text, multiline text with `\n` | `""`, whitespace-only (`"   "`), ASCII control chars (`\0`) |
| `canonical` | Complete | `"http://hl7.org/fhir/StructureDefinition/Patient"`, `"uri\|1.0#frag"`, `""` | Internal whitespace |
| `code` | Complete | `"active"`, `"oral-route"`, `"code with single spaces"` | `""`, `" leading"`, `"trailing "`, `"double  spaces"`, `"\t"` |
| `date` | Complete | `"2020"`, `"2020-05"`, `"2020-05-15"` | `"2020-5"`, `"2020-13-01"`, `"2020-02-30"`, `"0000"`, `"2020-00"` |
| `dateTime` | Complete | `"2020"`, `"2020-05-15T10:30:00Z"`, `"2020-05-15T10:30:00+02:00"` | Missing timezone on time (`"2020-05-15T10:30:00"`), bad leap year |
| `instant` | Complete | `"2020-05-15T10:30:00Z"`, `"2020-05-15T10:30:00.123456789+05:30"` | Partial date (`"2020-05-15"`), missing seconds, offset `> 14:00` |
| `markdown` | Complete | `"# Header\n\n**bold**"` | `""`, `"   "` (same non-whitespace-content rule as `string`), strings exceeding 1,048,576 characters |
| `oid` | Complete | `"urn:oid:1.2.3.4"`, `"urn:oid:2.999.1"` | `"1.2.3.4"` (missing prefix), `"urn:oid:3.1"`, `"urn:oid:1.01"` |
| `positiveInt`| Complete | `1`, `42`, `2147483647`, `"1"` | `0`, `-1`, `"+1"`, `"01"`, `2147483648`, `""` |
| `time` | Complete | `"00:00:00"`, `"23:59:59"`, `"14:30:00.123"`, `"23:59:60"` | `"24:00:00"`, `"12:60:00"`, `"12:00"`, `"12"`, `"-01:00:00"` |
| `unsignedInt`| Complete | `0`, `1`, `2147483647`, `"0"`, `"1"` | `-1`, `"-0"`, `"+0"`, `"01"`, `2147483648`, `""` |
| `uri` | Complete | `"http://example.org"`, `"urn:uuid:123"`, `"/relative/path"`, `""` | Internal whitespace |
| `url` | Complete | `"http://example.org/index.html"`, `"https://fhir.org"`, `""` | Internal whitespace, non-URL characters |
| `uuid` | Complete | `"urn:uuid:c707a726-25f0-466d-9788-b2ef562d4e78"` | Upper case (`"urn:uuid:C707..."`), missing `"urn:uuid:"` prefix |

---

## 4. Remaining Work Order

All 20 implemented FHIR R5 primitives are complete (`xhtml` not yet implemented). Seven
complex types are complete: `Extension`, `Period`, `Coding`, `Quantity`,
`CodeableConcept`, `Identifier`, `Reference`. 34 further complex/metadata/
special-purpose types exist as doc-only stubs with no tests (nothing to test yet — see
`docs/BACKLOG.md` Roadmap for build order).

### 4.1 `Primitive<T>` Test Coverage (`src/datatypes/primitive/test.rs`)

- `ele-1`: value-only succeeds, extension-only succeeds, both succeeds, neither fails,
  **`id` alone fails** (the FHIRPath nuance — `id` doesn't count on its own).
- `from_value` is infallible and produces a bare (no id/extension) instance.
- `new_unchecked` bypasses `ele-1`.
- Accessors, `Clone`/`PartialEq`.

### 4.2 `Extension` Test Coverage (`src/datatypes/complex/extension/test.rs`)

- `ext-1` XOR enforcement: value-only succeeds, children-only succeeds, neither fails,
  both fails (asserts the exact `ConstraintError::InvariantViolated` message).
- Nested/recursive extensions (grandchild depth).
- `id`/`url`/`extensions`/`value` accessors — `url()` now returns `&Primitive<Uri>`.
- `new_unchecked` bypasses `ext-1` (mirrors every primitive's escape hatch).
- One round-trip per `ExtensionValue` variant (all 20 primitives plus `Period`,
  `Coding`, `Quantity`, `CodeableConcept`, `Identifier`, `Reference`), each
  `Primitive`-wrapped (primitives) or embedded directly (complex types — no companion
  split for those).
- Serde: value-only, id+children, value-with-companion, companion-only-no-value
  (data-absent-reason) for both `url` and `value[x]`, `_valueX` merged regardless of
  JSON key order, missing `url` fails, `ext-1` violations fail on deserialize,
  unknown fields ignored, `id`-alone-on-a-companion fails `ele-1` on deserialize.

### 4.3 Complex Type Test Coverage (`Period`, `Coding`, `Quantity`, `CodeableConcept`, `Identifier`, `Reference`)

Each has its own `test.rs` following the same shape: happy path (individual fields, all
fields together, extension-only), negative/boundary (its error-severity invariant
violated — `ele-1` for `Period`/`Coding`/`CodeableConcept`/`Identifier`, `qty-3` for
`Quantity`, `ref-2` for `Reference`, plus `ref-2`'s `type`-alone-insufficient case),
invalid nested field values, and malformed/non-object JSON — for both direct
construction and serde. Warning-severity invariants (`cod-1`, `ident-1`) and
not-implementable-here invariants (`per-1`, `ref-1`) are documented as deliberately
unenforced, with a test confirming construction still succeeds despite violating them
(e.g. `test_new_display_without_code_succeeds_cod1_is_warning_only`).

Next: pick the next complex-type stub to implement — all 7 originally-scoped types are
now done, so this means one of the 34 further stubs (see `docs/BACKLOG.md` Roadmap for
the list) — and design only the construction/validation/serde surface its confirmed
error-severity invariants need. Not scheduled further than that until it's the current
task.
