# Low-Level Design (LLD): `fhir-core` Type System

**Version:** 0.1.0
**FHIR Specification:** Release 5 (R5) — [HL7 FHIR R5](https://hl7.org/fhir/R5/)
**Crate Safety Invariants:** `#![forbid(unsafe_code)]`, zero non-serde runtime dependencies.

---

## 1. Scope

Current scope is the 20 FHIR R5 primitive types as standalone Rust structs — no trait
hierarchy, no `Element`/`Resource` model, no complex types. Those are deferred until a
concrete consumer needs them (§4); building them ahead of a consumer produced a
speculative, self-contradicting design in a previous draft of this document and is not
repeated here.

### Core Design Principles

1. **Spec-Driven Strictness (HL7 FHIR R5 Normative)**: every primitive format, regex,
   numeric range is verified directly against the official HL7 FHIR R5 spec pages, not
   recalled from memory.
2. **Zero-Cost & Zero-Allocation Views**: accessors return slices (`&str`) and native
   primitives (`f64`, `i32`, `i64`, `bool`) without cloning.
3. **Ergonomic Serde Integration**: direct JSON compatibility, zero non-serde deps.
   Correct handling of spec peculiarities (e.g. `integer64` as a JSON string).
4. **Compile-Time Safety & Explicit Invariants**: `#![forbid(unsafe_code)]` crate-wide.
   No implicit conversions that bypass validation (no `Deref<Target = str>`). Standard
   conversion traits: `TryFrom<&str>`, `TryFrom<String>`, `FromStr`, `Display`,
   `From`/`Into` for the wrapped native type.

---

## 2. Primitive Type Pattern

Every primitive in `src/types/<type_name>/` follows the same shape (see
`src/types/decimal/mod.rs` for the most fully worked example):

- Wraps the native Rust type, or the validated string itself when spec semantics can't
  be captured natively (e.g. `Decimal` preserving trailing-zero precision).
- `new(val)` — infallible when all native values are valid; otherwise fallible.
- `new_unchecked(val)` — skips validation.
- `validate(&str) -> Result<(), TypeError>` performing the FHIR-spec grammar check.
- `TryFrom<&str>`, `TryFrom<String>`, `FromStr`, `Display`, `From`/`Into` conversions.
- No `Deref`/`DerefMut` — explicit accessors only (`as_str()`, `as_bool()`, `as_f64()`,
  `into_inner()`, ...).
- Serde via `#[derive(Serialize, Deserialize)]` or explicit impls when the JSON
  encoding differs from the Rust representation.

---

## 3. Primitives Specification Matrix

| Primitive | Rust Type | Representation | HL7 R5 Pattern / Invariant | Serde JSON Encoding | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `base64Binary` | `Base64Binary` | `String` | RFC 4648 Base64: `(\s*([0-9a-zA-Z+/=]){4}\s*)+` | JSON string | Complete |
| `boolean` | `Boolean` | `bool` | `true \| false` | JSON boolean | Complete |
| `decimal` | `Decimal` | `String` | `-?(0\|[1-9][0-9]{0,17})(\.[0-9]{1,17})?([eE][+-]?[0-9]{1,9})?` | JSON number | Complete |
| `id` | `Id` | `String` | `[A-Za-z0-9\-\.]{1,64}` | JSON string | Complete |
| `integer` | `Integer` | `i32` | `[0]\|[+-]?[1-9][0-9]*`, no signed zero, $-2^{31}..2^{31}-1$ | JSON number | Complete |
| `integer64` | `Integer64` | `i64` | `[0]\|[+-]?[1-9][0-9]*`, no signed zero, $-2^{63}..2^{63}-1$ | **JSON string** (per spec) | Complete |
| `string` | `FhirString` | `String` | `[ \r\n\t\S]+`, max 1MB. Control chars rejected as a hard MUST (deliberate strengthening of the spec's SHOULD-NOT) | JSON string | Complete |
| `canonical` | `Canonical` | `String` | RFC 3986 URI + optional `\|version` / `#frag` (`\S*`, empty allowed) | JSON string | Pending |
| `code` | `Code` | `String` | `[^\s]+( [^\s]+)*` (no lead/trail ws, only literal single spaces between tokens) | JSON string | Complete |
| `date` | `Date` | `String` | `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`, calendar-valid | JSON string | Complete |
| `dateTime` | `DateTime` | `String` | `YYYY`, `YYYY-MM`, `YYYY-MM-DD`, or ISO-8601 with mandatory TZ when time is present | JSON string | Complete |
| `instant` | `Instant` | `String` | `YYYY-MM-DDThh:mm:ss[.sss](Z\|[+-]hh:mm)` (sec + TZ required) | JSON string | Pending |
| `markdown` | `Markdown` | `String` | Reuses `string`'s pattern (`[ \r\n\t\S]+`), max 1,048,576 chars | JSON string | Pending |
| `oid` | `Oid` | `String` | `urn:oid:[0-2](\.(0\|[1-9][0-9]*))+` | JSON string | Pending |
| `positiveInt`| `PositiveInt`| `u32` (range $\ge 1$)| `[1-9][0-9]*`, $1..2147483647$ | JSON number | Pending |
| `time` | `Time` | `String` | `hh:mm:ss[.sss]` (leap seconds allowed) | JSON string | Pending |
| `unsignedInt`| `UnsignedInt`| `u32` (range $\ge 0$)| `[0]\|([1-9][0-9]*)`, $0..2147483647$ | JSON number | Pending |
| `uri` | `Uri` | `String` | RFC 3986 URI: `\S*`, empty allowed | JSON string | Pending |
| `url` | `Url` | `String` | RFC 1738/3986 URL: `\S*`, empty allowed | JSON string | Pending |
| `uuid` | `Uuid` | `String` | `urn:uuid:[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}` | JSON string | Pending |

Every regex/range above must be re-verified against hl7.org/fhir/R5/datatypes.html
before implementing — this table is a working reference, not a substitute for the
spec page.

---

## 4. Deferred: Element Model & Complex Types

Not designed yet, on purpose: `Base`/`Element`/`Resource` trait hierarchy, `Primitive<T>`
wrapper, `_propertyName` companion JSON handling, and complex types (`Extension`,
`Coding`, `CodeableConcept`, `Identifier`, `Period`, `Quantity`, `Reference`) and choice
type enums (`ExtensionValue`).

These get designed bottom-up when a real consumer needs them — e.g. `Extension` is the
natural first complex type, and it should pull in only the trait(s)/wrapper it actually
requires, not the full hierarchy up front. Building the full hierarchy speculatively
last time produced code that didn't compile (`&[Resource]` with an unsized trait) and a
`Primitive<T>` with public fields that let its own `ele-1` invariant be bypassed by
direct construction.

---

## 5. Error Handling

Current, shipped:

```
FhirCoreError
  └─ Type(TypeError)
       └─ TypeError::InvalidValue { r#type: String, value: String, error: String }
```

`ConstraintError` (multi-field invariants like `ele-1`, `ext-1`, `per-1`) and any
serialization-specific error variant are scoped and added alongside whichever complex
type first needs them — not before.

---

## 6. Directory Layout

```
fhir-core/
├── Cargo.toml
├── Makefile
├── docs/
│   ├── LLD.md
│   ├── TDD.md
│   └── TECH_DESIGN.md
└── src/
    ├── lib.rs
    ├── errors/
    │   ├── mod.rs           <-- FhirCoreError
    │   ├── type.rs           <-- TypeError
    │   ├── constraints.rs    <-- stub, unused until a complex type needs it
    │   └── test.rs
    ├── types/                <-- FHIR R5 primitives, one dir per type
    │   ├── mod.rs
    │   ├── base64/           {mod.rs, test.rs}   Complete
    │   ├── boolean/          {mod.rs, test.rs}   Complete
    │   ├── code/             {mod.rs, test.rs}   Complete
    │   ├── date/             {mod.rs, test.rs}   Complete
    │   ├── date_time/        {mod.rs, test.rs}   Complete (FHIR `dateTime`)
    │   ├── decimal/          {mod.rs, test.rs}   Complete
    │   ├── id/                {mod.rs, test.rs}   Complete
    │   ├── integer/           {mod.rs, test.rs}   Complete
    │   ├── integer64/         {mod.rs, test.rs}   Complete
    │   ├── string/            {mod.rs, test.rs}   Complete
    │   └── (11 remaining: canonical, instant, markdown, oid,
    │        positiveInt, time, unsignedInt, uri, url, uuid)
    └── r5/                    <-- placeholder for future resource models
```
