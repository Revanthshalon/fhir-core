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
| `canonical` | `Canonical` | `String` | RFC 3986 URI + optional `\|version` / `#frag` (`\S*`, empty allowed) | JSON string | Complete |
| `code` | `Code` | `String` | `[^\s]+( [^\s]+)*` (no lead/trail ws, only literal single spaces between tokens) | JSON string | Complete |
| `date` | `Date` | `String` | `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`, calendar-valid | JSON string | Complete |
| `dateTime` | `DateTime` | `String` | `YYYY`, `YYYY-MM`, `YYYY-MM-DD`, or ISO-8601 with mandatory TZ when time is present | JSON string | Complete |
| `instant` | `Instant` | `String` | `YYYY-MM-DDThh:mm:ss[.sss](Z\|[+-]hh:mm)` (full precision + TZ mandatory, no partial dates) | JSON string | Complete |
| `markdown` | `Markdown` | `String` | Reuses `string`'s pattern (`[ \r\n\t\S]+`), max 1,048,576 chars | JSON string | Complete |
| `oid` | `Oid` | `String` | `urn:oid:[0-2](\.(0\|[1-9][0-9]*))+` | JSON string | Complete |
| `positiveInt`| `PositiveInt`| `u32` (range $\ge 1$)| `[1-9][0-9]*`, $1..2147483647$ | JSON number | Complete |
| `time` | `Time` | `String` | `hh:mm:ss[.sss]` (leap seconds allowed, no `24:00`, no timezone) | JSON string | Complete |
| `unsignedInt`| `UnsignedInt`| `u32` (range $\ge 0$)| `[0]\|([1-9][0-9]*)`, $0..2147483647$ | JSON number | Complete |
| `uri` | `Uri` | `String` | RFC 3986 URI: `\S*`, empty allowed | JSON string | Complete |
| `url` | `Url` | `String` | RFC 1738/3986 URL: `\S*`, empty allowed | JSON string | Complete |
| `uuid` | `Uuid` | `String` | `urn:uuid:[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}` | JSON string | Complete |

Every regex/range above must be re-verified against hl7.org/fhir/R5/datatypes.html
before implementing — this table is a working reference, not a substitute for the
spec page.

---

## 4. Deferred: Element Model & Remaining Complex Types

`Extension` (`src/datatypes/complex/extension/`) and `Primitive<T>`
(`src/datatypes/primitive/`) are built — see §4.1 and §4.2. Still not designed, on
purpose: `Base`/`Element`/`Resource` trait hierarchy and the remaining complex types
(`Coding`, `CodeableConcept`, `Identifier`, `Period`, `Quantity`, `Reference`).

These get designed bottom-up when a real consumer needs them, pulling in only the
trait(s)/wrapper that consumer actually requires — not a hierarchy up front. Building
the full hierarchy speculatively last time produced code that didn't compile
(`&[Resource]` with an unsized trait) and a `Primitive<T>` with public fields that let
its own `ele-1` invariant be bypassed by direct construction — the private-fields +
validating-constructor shape both `Extension` and the real `Primitive<T>` use now is a
direct reaction to that.

### 4.1 `Extension`

No traits: nothing consumes `Extension` polymorphically yet, so it's a plain struct with
private fields, a validating constructor (`Extension::new`, checks `ext-1`), and
`new_unchecked` as the explicit bypass — the same shape every primitive already uses.

`ExtensionValue` (`value[x]`) covers only the 20 primitives this crate has implemented,
not the full 54 types the real spec allows (20 primitives + 35 complex types). A
complex-type variant is added the moment that complex type is built, not before —
`Extension` becomes the natural second consumer for each one as the crate grows.

`Extension.url` and every `ExtensionValue` variant wrap
[`Primitive<T>`](#42-primitivet), not the bare primitive type, since both are
primitive-valued properties eligible for the JSON companion pattern (§4.2).
`Extension.id` stays a plain `FhirString` — the spec special-cases `Element.id` as an
ordinary property, never itself companion-wrapped.

#### Known gap: unrecognized `value[x]` is dropped on deserialize

Any `value*`/`_value*` JSON key this crate doesn't yet model (one of the 34 remaining
complex types, or a future primitive) is currently discarded via `IgnoredAny` in
`Extension`'s `Deserialize` impl — deserializing then re-serializing such an `Extension`
loses that value. Verified against spec (hl7.org/fhir/R5/extensibility.html §2.1.5.0.3):
retention is a **SHOULD** ("systems SHOULD retain unknown extensions when they are
capable of doing so"), not a MUST — there is no hard lossless round-trip mandate, so this
is a capability gap, not a spec violation.

Considered fix: an `ExtensionValue::Unknown(String)` catch-all capturing the raw scalar
value. Deferred — it only covers scalar `value*` shapes, not the object-shaped complex
types that make up most of the gap, and it conflicts with this crate's pattern of every
`ExtensionValue` variant being a validated newtype rather than an untyped escape hatch.
Revisit once the first complex type lands (§4.1's "natural second consumer" point) —
either the complex-type variants close the gap directly, or an explicit decision to add
an `Unknown` fallback gets made then, with real callers to judge the tradeoff against.

### 4.2 `Primitive<T>`

`{ value: Option<T>, id: Option<FhirString>, extension: Vec<Extension> }` — what every
FHIR primitive-valued property structurally is. Private fields; `Primitive::new`
validates `ele-1`, `Primitive::from_value` is the infallible "just a bare value" sugar,
`new_unchecked` is the explicit bypass.

`ele-1`'s FHIRPath is `hasValue() or (children().count() > id.count())` — `id` counts as
a child, so `id` alone (no value, no extension) does **not** satisfy `ele-1`. This is a
real spec nuance, not the more obvious-looking "value or id or extension" — caught by
fetching hl7.org/fhir/R5/types.html#Element rather than assuming, per this crate's
spec-driven-not-memory-driven rule.

**No `Serialize`/`Deserialize` on `Primitive<T>` itself** — it isn't representable as a
single JSON value in general (it can be 0, 1, or 2 sibling keys: bare `propertyName`,
`_propertyName` companion, or both, depending on state). The split/merge logic lives in
two `pub(crate)` helpers next to the type (`serialize_primitive_entry`,
`merge_primitive_entry`), used by every hand-rolled container `Serialize`/`Deserialize`
impl — currently just `Extension`'s (21 call sites: `url` + 20 `value[x]` variants).

---

## 5. Error Handling

Current, shipped:

```
FhirCoreError
  ├─ Type(TypeError)
  │    └─ TypeError::InvalidValue { r#type: String, value: String, error: String }
  └─ Constraint(ConstraintError)
       └─ ConstraintError::InvariantViolated { key: &'static str, description: String }
```

`ConstraintError` was added alongside `Extension`, the first complex type needing a
multi-field invariant (`ext-1`). Remaining invariants (`ele-1`, `per-1`, ...) get their
own `InvariantViolated { key, .. }` call sites as the types that need them are built —
no new error variant required, `key` already carries the invariant identity. Any
serialization-specific error variant is still scoped to whichever future need requires
it — not before.

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
    │   ├── constraints.rs    <-- ConstraintError (ext-1; more invariants reuse it)
    │   └── test.rs
    ├── types/                <-- FHIR R5 primitives, one dir per type
    │   ├── mod.rs
    │   ├── base64/           {mod.rs, test.rs}   Complete
    │   ├── boolean/          {mod.rs, test.rs}   Complete
    │   ├── canonical/        {mod.rs, test.rs}   Complete
    │   ├── code/             {mod.rs, test.rs}   Complete
    │   ├── date/             {mod.rs, test.rs}   Complete
    │   ├── date_time/        {mod.rs, test.rs}   Complete (FHIR `dateTime`)
    │   ├── decimal/          {mod.rs, test.rs}   Complete
    │   ├── id/                {mod.rs, test.rs}   Complete
    │   ├── instant/           {mod.rs, test.rs}   Complete
    │   ├── integer/           {mod.rs, test.rs}   Complete
    │   ├── integer64/         {mod.rs, test.rs}   Complete
    │   ├── markdown/          {mod.rs, test.rs}   Complete
    │   ├── oid/               {mod.rs, test.rs}   Complete
    │   ├── positive_int/      {mod.rs, test.rs}   Complete (FHIR `positiveInt`)
    │   ├── string/            {mod.rs, test.rs}   Complete
    │   ├── time/              {mod.rs, test.rs}   Complete
    │   ├── unsigned_int/      {mod.rs, test.rs}   Complete (FHIR `unsignedInt`)
    │   ├── uri/               {mod.rs, test.rs}   Complete
    │   ├── url/               {mod.rs, test.rs}   Complete
    │   ├── uuid/              {mod.rs, test.rs}   Complete
    │   └── (all 20 FHIR R5 primitives complete)
    ├── datatypes/             <-- FHIR reusable data types
    │   ├── mod.rs
    │   ├── primitive/         {mod.rs, test.rs}   Complete (Primitive<T>)
    │   └── complex/
    │       ├── mod.rs
    │       └── extension/     {mod.rs, test.rs}   Complete
    └── r5/                    <-- placeholder for future resource models
```
