# Low-Level Design (LLD): `fhir-core` Type System

**Version:** 0.1.0
**FHIR Specification:** Release 5 (R5) — [HL7 FHIR R5](https://hl7.org/fhir/R5/)
**Crate Safety Invariants:** `#![forbid(unsafe_code)]`, zero non-serde runtime dependencies.

---

## 1. Scope

Current scope: 20 of the 21 FHIR R5 primitive types (`xhtml` not yet implemented) as
standalone Rust structs, plus seven fully implemented complex types (`Extension`,
`Period`, `Coding`, `Quantity`, `CodeableConcept`, `Identifier`, `Reference` — see §4)
and 34 further complex/metadata/special-purpose types as doc-only stubs (real fields,
no construction/validation/serde). No `Base`/`Element`/`Resource` trait hierarchy yet.
Anything past the seven implemented complex types is deferred until a concrete consumer
needs it (§4); building the full hierarchy ahead of a consumer produced a speculative,
self-contradicting design in a previous draft of this document and is not repeated here.

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
- `validate(&str) -> Result<(), XError>` performing the FHIR-spec grammar check, where
  `XError` is that primitive's own granular error enum (`DateError`, `Base64Error`,
  ...) — re-exported from `src/types/` so external callers can name and match on it.
  `TryFrom`/`FromStr` map that into the crate-wide `TypeError` (§5). Four primitives
  (`Integer`, `Integer64`, `PositiveInt`, `UnsignedInt`) also expose `parse(&str) ->
  Result<NativeType, XError>` for callers who want the parsed value directly, since
  `validate` itself is `Result<(), XError>` for signature symmetry across all 19.
- `TryFrom<&str>`, `TryFrom<String>`, `FromStr`, `Display`, `From`/`Into` conversions.
- No `Deref`/`DerefMut` — explicit accessors only (`as_str()`, `as_bool()`, `as_f64()`,
  `into_inner()`, ...).
- Serde via `#[derive(Serialize, Deserialize)]` or explicit impls when the JSON
  encoding differs from the Rust representation.

---

## 3. Primitives Specification Matrix

| Primitive | Rust Type | Representation | HL7 R5 Pattern / Invariant | Serde JSON Encoding | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `base64Binary` | `Base64Binary` | `String` | RFC 4648 Base64, R5 grammar: `(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==\|[A-Za-z0-9+/]{3}=)?` — no whitespace, empty allowed. (R4's grammar differs — permits internal whitespace, `(\s*([0-9a-zA-Z+/=]){4}\s*)+` — and is not yet implemented; see `docs/BACKLOG.md`.) | JSON string | Complete (R5 only) |
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

## 4. Complex Types & the Element Model

`Primitive<T>` (`src/datatypes/primitive/`, §4.2) and seven complex types are built:
`Extension` (§4.1), `Period`, `Coding`, `Quantity`, `CodeableConcept`, `Identifier`,
`Reference` (`src/datatypes/complex/{period,coding,quantity,codeable_concept,
identifier,reference}/`). Each follows the same shape as `Extension` (§4.1): private
fields, `id`/`extension`, a validating constructor for its confirmed error-severity
invariants (only `ele-1` for most; `qty-3` for `Quantity`; `ref-2` for `Reference`),
accessors, and hand-rolled `Serialize`/`Deserialize` mirroring `Extension`'s
bare-value/companion split for primitive fields (complex-typed fields embed directly,
no companion). `Identifier` and `Reference` are mutually recursive (each embeds the
other via `Box`) and were designed together for that reason.

Still not designed, on purpose: the `Base`/`Element`/`Resource` trait hierarchy, and
real implementations of the 34 further complex/metadata/special-purpose types beyond
the seven above (doc-only stubs exist for all of them — see `docs/BACKLOG.md` Roadmap).

These get designed bottom-up when a real consumer needs them, pulling in only the
trait(s)/wrapper that consumer actually requires — not a hierarchy up front. Building
the full hierarchy speculatively last time produced code that didn't compile
(`&[Resource]` with an unsized trait) and a `Primitive<T>` with public fields that let
its own `ele-1` invariant be bypassed by direct construction — the private-fields +
validating-constructor shape both `Extension` and the real `Primitive<T>` use now is a
direct reaction to that.

Several invariants confirmed to exist were initially missed because the check only
looked at hl7.org/fhir/R5/datatypes.html, which doesn't list invariants inline — the
full text lives on hl7.org/fhir/R5/datatypes-definitions.html (or
metadatatypes-definitions.html / elementdefinition-definitions.html for those
categories). `Period`'s `per-1`, `Coding`'s `cod-1`, and `Quantity`'s `qty-3` were all
caught this way; treat "no named invariants found" on any stub as unverified until the
definitions page has actually been checked, not the summary page.

### 4.1 `Extension`

No traits: nothing consumes `Extension` polymorphically yet, so it's a plain struct with
private fields, a validating constructor (`Extension::new`, checks `ext-1`), and
`new_unchecked` as the explicit bypass — the same shape every primitive already uses.

`ExtensionValue` (`value[x]`) covers the 20 implemented primitives plus `Period`,
`Coding`, `Quantity`, `CodeableConcept`, `Identifier`, and `Reference` — not the full 54
types the real spec allows. A complex-type variant is added the moment that complex
type is built, not before — `Extension` becomes the natural second consumer for each
one as the crate grows (all six built so far have gained a variant this way).

`Extension.url` and every `ExtensionValue` variant wrap
[`Primitive<T>`](#42-primitivet), not the bare primitive type, since both are
primitive-valued properties eligible for the JSON companion pattern (§4.2).
`Extension.id` stays a plain `FhirString` — the spec special-cases `Element.id` as an
ordinary property, never itself companion-wrapped.

#### Known gap: unrecognized `value[x]` is dropped on deserialize

Any `value*`/`_value*` JSON key this crate doesn't yet model (one of the ~28 complex
types still uncovered, or the unimplemented `xhtml` primitive) is currently discarded
via `IgnoredAny` in
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
impl — all seven complex types now (`Extension`'s own 21 call sites: `url` + 20
primitive `value[x]` variants, plus one call site per primitive-typed field on
`Period`/`Coding`/`Quantity`/`CodeableConcept`/`Identifier`/`Reference`). The six
complex-typed `ExtensionValue` variants (`Period`, `Coding`, ...) skip these helpers
entirely — they embed as a single JSON object with no companion split, since only
primitives get that treatment.

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
multi-field invariant (`ext-1`). Confirmed-implemented invariants across the seven
complex types each got their own `InvariantViolated { key, .. }` call site as that type
was built (`ele-1`, `ext-1`, `qty-3`, `ref-2`) — no new error variant required, `key`
already carries the invariant identity. Warning-severity invariants (`cod-1`,
`ident-1`) and not-implementable-here ones (`per-1`, `ref-1`) deliberately have no
`ConstraintError` call site at all — see each type's module doc for why. Any
serialization-specific error variant is still scoped to whichever future need requires
it — not before.

Each primitive also has its own granular error enum (`DateError`, `Base64Error`, ...),
returned by that primitive's `validate`/`parse` and mapped into `TypeError::InvalidValue`
by `TryFrom`. These are re-exported from `src/types/` (not just left `pub` inside their
private `mod`) so external callers can name and pattern-match on the specific error, not
only the crate-wide `TypeError`.

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
    │   └── (20 of 21 FHIR R5 primitives complete; `xhtml` not yet implemented)
    ├── datatypes/             <-- FHIR reusable data types
    │   ├── mod.rs
    │   ├── primitive/         {mod.rs, test.rs}   Complete (Primitive<T>)
    │   └── complex/
    │       ├── mod.rs
    │       ├── extension/            {mod.rs, test.rs}   Complete
    │       ├── period/               {mod.rs, test.rs}   Complete
    │       ├── coding/               {mod.rs, test.rs}   Complete
    │       ├── quantity/             {mod.rs, test.rs}   Complete
    │       ├── codeable_concept/     {mod.rs, test.rs}   Complete
    │       ├── identifier/           {mod.rs, test.rs}   Complete
    │       ├── reference/            {mod.rs, test.rs}   Complete
    │       └── (34 further complex/metadata/special-purpose types: doc-only stub
    │            mod.rs, no test.rs — real fields, no construction/validation/serde.
    │            Private modules, not re-exported. See docs/BACKLOG.md Roadmap.)
    └── r5/                    <-- wired in lib.rs behind feature = "r5";
                                    placeholder for future resource models
```
