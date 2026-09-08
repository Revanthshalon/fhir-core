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

Layers for invariant checks (`ele-1`, `ext-1`, `per-1`, ...) and the `_propertyName`
companion protocol don't apply yet — they're specified alongside whichever complex
type first needs them (see LLD.md §4), not ahead of time.

---

## 3. Primitives Test Matrix

| Primitive | Status | Happy Path Vectors | Negative / Edge Vectors |
| :--- | :--- | :--- | :--- |
| `base64Binary` | Complete | `"TWFu"`, `"AAAA"`, with internal whitespace | Odd lengths, illegal chars (`@`, `!`), invalid padding (`===`) |
| `boolean` | Complete | `true`, `false`, `"true"`, `"false"` | `"True"`, `"FALSE"`, `"yes"`, `"1"`, `""` |
| `decimal` | Complete | `"0"`, `"-0"`, `"0.010"`, `"42.5"`, `"1e3"`, `"-1.2E-3"` | `"+42"`, `".5"`, `"5."`, `"0123"`, non-numeric, finite check |
| `id` | Complete | `"a"`, `"patient-1234"`, `"Observation.1"`, 64-char string | `""`, 65-char string, spaces, `_`, `/`, `@`, unicode |
| `integer` | Complete | `0`, `42`, `-100`, `"+42"`, `"0"`, `i32::MIN`, `i32::MAX` | `"-0"`, `"+0"`, `"01"`, `i32::MAX + 1`, `42.5`, letters |
| `integer64` | Complete | `0`, `42`, `i64::MIN`, `i64::MAX`, serialized as `"42"` | `"-0"`, `"+0"`, `"01"`, `i64::MAX + 1`, raw JSON numbers |
| `string` | Complete | `"Hello"`, unicode text, multiline text with `\n` | `""`, whitespace-only (`"   "`), ASCII control chars (`\0`) |
| `canonical` | Pending | `"http://hl7.org/fhir/StructureDefinition/Patient"`, `"uri\|1.0#frag"`, `""` | Internal whitespace |
| `code` | Complete | `"active"`, `"oral-route"`, `"code with single spaces"` | `""`, `" leading"`, `"trailing "`, `"double  spaces"`, `"\t"` |
| `date` | Complete | `"2020"`, `"2020-05"`, `"2020-05-15"` | `"2020-5"`, `"2020-13-01"`, `"2020-02-30"`, `"0000"`, `"2020-00"` |
| `dateTime` | Complete | `"2020"`, `"2020-05-15T10:30:00Z"`, `"2020-05-15T10:30:00+02:00"` | Missing timezone on time (`"2020-05-15T10:30:00"`), bad leap year |
| `instant` | Complete | `"2020-05-15T10:30:00Z"`, `"2020-05-15T10:30:00.123456789+05:30"` | Partial date (`"2020-05-15"`), missing seconds, offset `> 14:00` |
| `markdown` | Pending | `"# Header\n\n**bold**"`, `"   "`, `""` (verify against spec — may share `string`'s emptiness rule or diverge) | Strings exceeding 1,048,576 characters |
| `oid` | Complete | `"urn:oid:1.2.3.4"`, `"urn:oid:2.999.1"` | `"1.2.3.4"` (missing prefix), `"urn:oid:3.1"`, `"urn:oid:1.01"` |
| `positiveInt`| Complete | `1`, `42`, `2147483647`, `"1"` | `0`, `-1`, `"+1"`, `"01"`, `2147483648`, `""` |
| `time` | Complete | `"00:00:00"`, `"23:59:59"`, `"14:30:00.123"`, `"23:59:60"` | `"24:00:00"`, `"12:60:00"`, `"12:00"`, `"12"`, `"-01:00:00"` |
| `unsignedInt`| Complete | `0`, `1`, `2147483647`, `"0"`, `"1"` | `-1`, `"-0"`, `"+0"`, `"01"`, `2147483648`, `""` |
| `uri` | Complete | `"http://example.org"`, `"urn:uuid:123"`, `"/relative/path"`, `""` | Internal whitespace |
| `url` | Complete | `"http://example.org/index.html"`, `"https://fhir.org"`, `""` | Internal whitespace, non-URL characters |
| `uuid` | Complete | `"urn:uuid:c707a726-25f0-466d-9788-b2ef562d4e78"` | Upper case (`"urn:uuid:C707..."`), missing `"urn:uuid:"` prefix |

---

## 4. Remaining Work Order

Straight list, no dates — one primitive at a time, `make check` green before moving on:

1. `canonical`
2. `markdown`

After all 20 primitives are done: pick the first real complex-type consumer (likely
`Extension`) and design only the trait/wrapper surface it needs — see LLD.md §4. Not
scheduled further than that until it's the current task.
