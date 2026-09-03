//! A module for the FHIR `decimal` primitive data type.
//!
//! **Not yet implemented.** This file exists to record the design rationale below so it is
//! accounted for when `Decimal` is built, rather than rediscovered mid-implementation.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#decimal)
//! - XML representation: union of `xs:decimal` and `xs:double`.
//! - Regex: `-?(0|[1-9][0-9]{0,17})(\.[0-9]{1,17})?([eE][+-]?[0-9]{1,9})?`
//!   - Unlike [`Integer`](crate::primitives::Integer)/[`Integer64`](crate::primitives::Integer64),
//!     **no leading `+` is permitted** — only an optional `-`.
//!   - Up to 18 significant integer digits, up to 17 fractional digits, optional exponent.
//! - JSON encoding: a JSON **number** (not a string, unlike `integer64`) — but see precision
//!   note below, which complicates using serde_json's default `f64`-backed `Number`.
//! - **Precision is semantically significant**: the spec states `0.010` is a different value
//!   to `0.01` (distinct measurement precision, not merely a formatting choice). Systems SHALL
//!   preserve the precision of the value as represented, for presentation and re-serialization.
//!
//! # Why this type needs a different shape than `Integer`/`Integer64`
//! For `Integer`, we deliberately rejected keeping the original parsed string alongside the
//! numeric value (see the `Integer` design discussion this module inherits from): a leading `+`
//! carries no semantic weight (`+42 == 42`), so a plain `Integer(i32)` newtype loses nothing
//! meaningful.
//!
//! `Decimal` is different: trailing zeros in the fractional part **are** meaningful per spec, and
//! a plain `f64`/big-decimal value cannot represent that distinction (`0.010` and `0.01` are the
//! same `f64`). To round-trip correctly, `Decimal` likely needs to retain something like:
//!
//! ```ignore
//! pub struct Decimal {
//!     // The exact validated string as received (source of truth for precision/formatting).
//!     raw: String,
//!     // A derived numeric value for arithmetic/comparison (e.g. a big-decimal type).
//!     value: SomeDecimalValue,
//! }
//! ```
//!
//! rather than a single-field newtype like `Integer(i32)`.
//!
//! # Open questions to resolve before implementing
//! - **Equality/ordering/hashing**: should `Decimal("0.010") == Decimal("0.01")` (equal by
//!   numeric value) or not (equal only by exact string, since precision differs per spec)? FHIR
//!   treats them as distinct values, so `PartialEq`/`Hash` most likely need to be string-based
//!   (or value+precision-based), not purely numeric — this is the opposite default from
//!   `Integer`.
//! - **Numeric backend**: `f64` cannot exactly represent all valid decimals (up to 18+17
//!   digits) and has no notion of trailing-zero precision. A big-decimal crate (or a
//!   hand-rolled digit-string representation) is likely required; evaluate `dep` budget/MSRV
//!   impact (this crate currently has zero non-serde dependencies) before adding one.
//! - **JSON round-trip**: serde_json's default `Number` is `f64`-backed and will not preserve
//!   trailing zeros or exact digit sequences on deserialize. Preserving precision through JSON
//!   likely requires either the `arbitrary_precision` serde_json feature or a custom
//!   `Deserialize` that captures the raw number token as text (via `deserialize_any` /
//!   `serde_json::value::RawValue`) instead of parsing straight to `f64`.
//! - **Mutability**: unlike `Integer::as_i32_mut`, an `as_f64_mut`-style accessor would be
//!   unsafe to expose here, since mutating the numeric value independently of `raw` would desync
//!   the two — mutation, if offered at all, should go through a constructor/setter that
//!   re-derives `raw` from the new value (losing original formatting) or is rejected in favor of
//!   always reconstructing via `TryFrom<&str>`/`new`.
//! - **Range/format validation**: reuse the sign/leading-zero/regex validation pattern from
//!   `Integer`/`Integer64`'s `validate()`, extended for the fractional part and exponent, and
//!   for rejecting `+` (which `integer`/`integer64` permit but `decimal` does not).

#[cfg(test)]
mod test;
