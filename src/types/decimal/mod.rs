//! A module for the FHIR `decimal` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#decimal)
//! - XML representation: union of `xs:decimal` and `xs:double`.
//! - Regex: `-?(0|[1-9][0-9]{0,17})(\.[0-9]{1,17})?([eE][+-]?[0-9]{1,9})?`
//!   - Unlike [`Integer`](crate::primitives::Integer)/[`Integer64`](crate::primitives::Integer64),
//!     **no leading `+` is permitted** — only an optional `-`.
//!   - Unlike `Integer`, the optional `-` wraps the *entire* alternation, not just the non-zero
//!     branch — so `"-0"` is syntactically valid for `decimal` (contrast with `Integer`, which
//!     rejects `"-0"`/`"+0"` because its regex only lets the sign precede `[1-9][0-9]*`).
//!   - Up to 18 significant integer digits, up to 17 fractional digits, optional exponent of up
//!     to 9 digits.
//! - JSON encoding: a JSON number (not a string, unlike `integer64`).
//! - **Precision is semantically significant**: the spec states `0.010` is a different value to
//!   `0.01` (distinct measurement precision, not merely a formatting choice). Systems SHALL
//!   preserve the precision of the value as represented, for presentation and re-serialization.
//!
//! # Design decisions and why
//!
//! **The wrapped value is the validated string itself (`Decimal(String)`), not a numeric type.**
//! Precision (trailing zeros) is part of the value's identity per spec, and no fixed-width
//! numeric type can represent that distinction (`f64` cannot tell `0.010` from `0.01`). The
//! string is validated once at construction and is otherwise treated as opaque/authoritative;
//! [`Decimal::as_f64`] derives a numeric view on demand for callers that need one, rather than
//! this type storing a second, independently-mutable numeric field that could desync from the
//! string.
//!
//! **`PartialEq`/`Eq`/`Hash` compare the raw string, not the numeric value.** This is what makes
//! `Decimal::try_from("0.010") != Decimal::try_from("0.01")` hold, matching the spec. A visible
//! consequence: this also makes `"100"` distinct from `"100.0"` and from `"1e2"` — FHIR only
//! specifies that trailing-zero *precision* is significant, not that alternate notations for the
//! same precision are distinct, so string-exact equality is intentionally slightly stricter than
//! the spec requires, in exchange for a much simpler and unambiguous equality rule. Values that
//! are numerically equal but written differently should be compared via
//! `a.as_f64() == b.as_f64()` explicitly if that's what's needed.
//!
//! **`PartialOrd`/`Ord` are deliberately NOT implemented**, making `Decimal` the only primitive
//! in this crate without them. Numeric ordering (via `as_f64`) is inconsistent with string-based
//! equality: `"0.010".partial_cmp("0.01")` would report `Equal` under numeric comparison while
//! `PartialEq` reports "not equal", violating the invariant (`a == b` iff `a.cmp(b) ==
//! Equal`) that sorting, `BTreeSet`, and `dedup` all rely on — silently dropping or misplacing
//! values that are precision-distinct but numerically equal. A consistent `Ord` would need to
//! order by `(numeric value, scale)` tuples rather than numeric value alone; that hasn't been
//! needed yet, so it's left unimplemented rather than guessed at. Callers who need numeric
//! ordering today can compare `a.as_f64().partial_cmp(&b.as_f64())` explicitly.
//!
//! **JSON round-tripping is lossy for fractional values, and that's an accepted, documented
//! limitation, not a bug.** `serde`'s data model has no primitive for "arbitrary-precision
//! number, exact digits preserved" — only fixed-width numeric types. `serde_json`'s tokenizer
//! converts a JSON number token like `4.5600` straight to an `f64` bit pattern *before* any
//! `Visitor` method of ours runs, so the trailing zeros are gone before our code ever sees them;
//! no `Deserialize` implementation can recover them without depending on `serde_json`'s
//! `arbitrary_precision` feature (a private, format-specific convention this crate deliberately
//! does not couple itself to, since it otherwise stays serde-format-agnostic). Consequently:
//! - Deserializing a whole-number JSON value (`42`, or a string `"42.00"`) preserves it exactly.
//! - Deserializing a fractional JSON *number* (`4.56`) reconstructs the string from the parsed
//!   `f64`, which cannot recover trailing zeros that were present in the original JSON text.
//! - Deserializing a JSON *string* (`"4.5600"`) preserves it exactly, since the exact text is
//!   handed to us directly — this is the precise path an XML-backed `serde` deserializer would
//!   also take, since XML element/attribute text always arrives as a string.
//! - Serializing prefers an exact integer encoding (`serialize_i64`) when the value has no
//!   fractional part or exponent, and falls back to `f64` otherwise — so only genuinely
//!   fractional values are subject to this loss.
//!
//! **Reconstructing a string from an `f64` never produces a string that fails [`Decimal::validate`].**
//! A magnitude like `1e300` formatted with Rust's plain `Display` produces a 301-digit integer
//! literal, which would exceed the 18-digit cap and make a spec-valid `Decimal` fail to
//! round-trip through its own deserializer. To avoid that, the `f64` reconstruction path checks
//! whether the plain-decimal form fits the FHIR digit limits and falls back to scientific
//! notation (`{:e}`) — which the FHIR regex explicitly allows — when it would not.
//!
//! **No dependency on an external bignum/decimal crate.** FHIR caps `decimal` at 18 integer + 17
//! fractional significant digits (35 total), comfortably within `i128`'s ~38-digit range, and
//! this type never needs to *compute* with arbitrary precision — only to validate, store, and
//! expose an `f64` view. A hand-rolled string validator is sufficient and keeps this crate at
//! zero non-serde dependencies.
//!
//! **[`Decimal::new_unchecked`] can make [`Decimal::as_f64`] panic.** Unlike
//! `Integer::new_unchecked`, which takes an already-valid `i32` and therefore cannot construct an
//! invalid state, `Decimal::new_unchecked` accepts an arbitrary string. `as_f64` assumes the
//! wrapped string satisfies [`Decimal::validate`]'s grammar (which is why it's guaranteed
//! `f64`-parseable) and will panic if that invariant was violated via `new_unchecked`. As with
//! [`crate::primitives::FhirString::new_unchecked`], the caller is responsible for the invariant.
//!
//! # Usage
//! To create a new [`Decimal`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`Decimal::new_unchecked`] when the input is already known to be valid.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::errors::r#type::TypeError;

#[cfg(test)]
mod test;

/// Represents a FHIR `decimal` primitive data type.
///
/// Wraps the exact, validated string representation of the value (see the module-level
/// documentation for why this is a string rather than a numeric type, and for the resulting
/// equality/ordering semantics).
///
/// # Examples
/// ```
/// use fhir_core::types::Decimal;
///
/// let d = Decimal::try_from("4.5600").unwrap();
/// assert_eq!(d.as_str(), "4.5600");
/// assert_eq!(d.as_f64(), 4.56);
///
/// // Precision is part of the value's identity.
/// let other = Decimal::try_from("4.56").unwrap();
/// assert_ne!(d, other);
///
/// let invalid = Decimal::try_from("+42");
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decimal(String);

/// Granular errors encountered while validating a FHIR `decimal` string representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecimalError {
    /// The string is empty.
    Empty,
    /// The string does not match the FHIR `decimal` grammar (e.g. a leading `+`, a bare `.5` or
    /// `5.` with no digits on one side of the point, non-digit characters, or trailing garbage).
    InvalidFormat,
    /// The integer part is non-zero but has a disallowed leading zero (e.g. `"0123.4"`).
    LeadingZero,
    /// The integer part has more than [`MAX_INTEGER_DIGITS`] (18) significant digits.
    TooManyIntegerDigits {
        /// The actual number of integer-part digits found.
        count: usize,
        /// The maximum permitted (18).
        max: usize,
    },
    /// The fractional part has more than [`MAX_FRACTION_DIGITS`] (17) digits.
    TooManyFractionDigits {
        /// The actual number of fractional-part digits found.
        count: usize,
        /// The maximum permitted (17).
        max: usize,
    },
    /// The exponent has more than [`MAX_EXPONENT_DIGITS`] (9) digits.
    TooManyExponentDigits {
        /// The actual number of exponent digits found.
        count: usize,
        /// The maximum permitted (9).
        max: usize,
    },
}

impl fmt::Display for DecimalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecimalError::Empty => write!(f, "decimal string must not be empty"),
            DecimalError::InvalidFormat => write!(f, "not a valid decimal representation"),
            DecimalError::LeadingZero => {
                write!(f, "non-zero decimal must not have a leading zero")
            }
            DecimalError::TooManyIntegerDigits { count, max } => write!(
                f,
                "integer part has {count} digits, exceeding the maximum of {max}"
            ),
            DecimalError::TooManyFractionDigits { count, max } => write!(
                f,
                "fractional part has {count} digits, exceeding the maximum of {max}"
            ),
            DecimalError::TooManyExponentDigits { count, max } => write!(
                f,
                "exponent has {count} digits, exceeding the maximum of {max}"
            ),
        }
    }
}

impl std::error::Error for DecimalError {}

impl Decimal {
    /// The maximum number of significant integer-part digits permitted by the FHIR `decimal` regex.
    const MAX_INTEGER_DIGITS: usize = 18;

    /// The maximum number of fractional-part digits permitted by the FHIR `decimal` regex.
    const MAX_FRACTION_DIGITS: usize = 17;

    /// The maximum number of exponent digits permitted by the FHIR `decimal` regex.
    const MAX_EXPONENT_DIGITS: usize = 9;

    /// Returns a string slice of the underlying validated value.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the underlying value parsed as an `f64`.
    ///
    /// This is a derived, on-demand numeric view — it is not stored, so it can never desync from
    /// [`Decimal::as_str`]. Because `f64` cannot represent every value the FHIR grammar permits
    /// exactly (up to 35 significant digits), this can lose precision for extreme values; the
    /// spec itself notes such values are rare in practice (e.g. high-precision `Location`
    /// coordinates).
    ///
    /// # Panics
    /// Panics if the wrapped string does not satisfy [`Decimal::validate`]'s grammar. This can
    /// only happen if the `Decimal` was constructed via [`Decimal::new_unchecked`] with an
    /// invalid string — every other constructor validates first.
    #[inline]
    pub fn as_f64(&self) -> f64 {
        self.0
            .parse()
            .expect("Decimal invariant violated: wrapped string is not valid decimal syntax")
    }

    /// Consumes the `Decimal` wrapper and returns the underlying validated `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `decimal` format:
    /// `-?(0|[1-9][0-9]{0,17})(\.[0-9]{1,17})?([eE][+-]?[0-9]{1,9})?`.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Decimal;
    ///
    /// assert!(Decimal::validate("0").is_ok());
    /// assert!(Decimal::validate("-0").is_ok());
    /// assert!(Decimal::validate("0.010").is_ok());
    /// assert!(Decimal::validate("+42").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<(), DecimalError> {
        if value.is_empty() {
            return Err(DecimalError::Empty);
        }

        let bytes = value.as_bytes();
        let mut i = 0;

        if bytes[i] == b'-' {
            i += 1;
        }

        let int_start = i;
        if i >= bytes.len() || !bytes[i].is_ascii_digit() {
            return Err(DecimalError::InvalidFormat);
        }
        if bytes[i] == b'0' {
            i += 1;
            if i < bytes.len() && bytes[i].is_ascii_digit() {
                return Err(DecimalError::LeadingZero);
            }
        } else {
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
        }
        let int_len = i - int_start;
        if int_len > Self::MAX_INTEGER_DIGITS {
            return Err(DecimalError::TooManyIntegerDigits {
                count: int_len,
                max: Self::MAX_INTEGER_DIGITS,
            });
        }

        if i < bytes.len() && bytes[i] == b'.' {
            i += 1;
            let frac_start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            let frac_len = i - frac_start;
            if frac_len == 0 {
                return Err(DecimalError::InvalidFormat);
            }
            if frac_len > Self::MAX_FRACTION_DIGITS {
                return Err(DecimalError::TooManyFractionDigits {
                    count: frac_len,
                    max: Self::MAX_FRACTION_DIGITS,
                });
            }
        }

        if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
            i += 1;
            if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
                i += 1;
            }
            let exp_start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            let exp_len = i - exp_start;
            if exp_len == 0 {
                return Err(DecimalError::InvalidFormat);
            }
            if exp_len > Self::MAX_EXPONENT_DIGITS {
                return Err(DecimalError::TooManyExponentDigits {
                    count: exp_len,
                    max: Self::MAX_EXPONENT_DIGITS,
                });
            }
        }

        if i != bytes.len() {
            return Err(DecimalError::InvalidFormat);
        }

        Ok(())
    }

    /// Creates a new `Decimal` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses the FHIR grammar validation. The caller is responsible for ensuring the
    /// provided value conforms to [`Decimal::validate`]. Violating this invariant does not cause
    /// undefined behavior, but it does make [`Decimal::as_f64`] panic.
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Reconstructs a FHIR-decimal-grammar string from an `f64`, used when deserializing a
    /// fractional JSON number (see the module-level documentation on why this path is lossy).
    ///
    /// Prefers plain decimal notation; falls back to scientific notation (which the FHIR regex
    /// permits) when the plain form would exceed the integer/fraction digit limits, so that a
    /// finite `f64` of any magnitude always reconstructs into a string [`Decimal::validate`]
    /// accepts.
    fn format_f64(value: f64) -> String {
        let plain = format!("{value}");
        let unsigned = plain.strip_prefix('-').unwrap_or(&plain);
        let (int_part, frac_part) = unsigned.split_once('.').unwrap_or((unsigned, ""));
        if int_part.len() <= Self::MAX_INTEGER_DIGITS
            && frac_part.len() <= Self::MAX_FRACTION_DIGITS
        {
            plain
        } else {
            format!("{value:e}")
        }
    }
}

impl TryFrom<&str> for Decimal {
    type Error = TypeError;

    /// Parses a `Decimal` from a string slice, validating the FHIR `decimal` grammar.
    ///
    /// # Errors
    /// Returns [`TypeError::InvalidValue`] if the input string does not conform to
    /// [`Decimal::validate`].
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value)
            .map(|()| Self(value.to_owned()))
            .map_err(|e| TypeError::InvalidValue {
                r#type: "decimal".to_owned(),
                value: value.to_owned(),
                error: e.to_string(),
            })
    }
}

impl TryFrom<String> for Decimal {
    type Error = TypeError;

    /// Parses a `Decimal` from an owned `String`.
    ///
    /// # Errors
    /// Returns [`TypeError::InvalidValue`] if the input string does not conform to
    /// [`Decimal::validate`].
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Decimal {
    type Err = TypeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for Decimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Prefer an exact integer encoding when there's no fraction/exponent to lose; only
        // genuinely fractional values fall through to the lossy f64 path (see module docs).
        if !self.0.contains(['.', 'e', 'E']) {
            if let Ok(i) = self.0.parse::<i64>() {
                return serializer.serialize_i64(i);
            }
        }

        serializer.serialize_f64(self.as_f64())
    }
}

struct DecimalVisitor;

impl de::Visitor<'_> for DecimalVisitor {
    type Value = Decimal;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "a FHIR decimal: a JSON number, or a string matching \
             -?(0|[1-9][0-9]{{0,17}})(\\.[0-9]{{1,17}})?([eE][+-]?[0-9]{{1,9}})?"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Decimal, E>
    where
        E: de::Error,
    {
        Decimal::validate(v)
            .map(|()| Decimal(v.to_owned()))
            .map_err(de::Error::custom)
    }

    fn visit_string<E>(self, v: String) -> Result<Decimal, E>
    where
        E: de::Error,
    {
        Decimal::validate(&v)
            .map(|()| Decimal(v))
            .map_err(de::Error::custom)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Decimal, E>
    where
        E: de::Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Decimal, E>
    where
        E: de::Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Decimal, E>
    where
        E: de::Error,
    {
        if !v.is_finite() {
            return Err(de::Error::custom(
                "decimal value must be finite (not NaN or infinite)",
            ));
        }
        self.visit_string(Decimal::format_f64(v))
    }
}

impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DecimalVisitor)
    }
}
