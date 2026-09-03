//! A module for the FHIR `integer64` primitive data type.
//!
//! This module provides an [`Integer64`] struct that encapsulates a signed 64-bit integer value
//! according to the FHIR specification.
//!
//! # Overview
//! The FHIR `integer64` primitive represents a signed 64-bit integer (corresponding to XML's
//! `xs:long`). Unlike [`Integer`](crate::primitives::Integer), it is defined to allow for
//! record/time counters that can get very large.
//!
//! Invariants according to the FHIR specification:
//! - Value SHALL be in the range -9,223,372,036,854,775,808..9,223,372,036,854,775,807
//!   (64-bit signed integer).
//! - String representation SHALL match the regex `[0]|[-+]?[1-9][0-9]*`:
//!   - `0` may stand alone.
//!   - Non-zero values MUST NOT have leading zeros.
//!   - An optional `+` or `-` sign is permitted.
//! - Unlike `integer`, `integer64` is transmitted as a JSON **string** (not a bare JSON number),
//!   because 64-bit integers can lose precision in floating-point JSON libraries.
//!
//! # Usage
//! To create a new [`Integer64`] instance:
//! - Use [`Integer64::new`] from an `i64` (infallible; every `i64` is a valid FHIR integer64).
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse from a string slice, which additionally
//!   validates the FHIR string format (rejecting leading zeros and malformed input).

use serde::{Deserialize, Serialize};

use crate::errors::r#type::TypeError;

#[cfg(test)]
mod test;

/// Represents a FHIR `integer64` primitive data type.
///
/// A signed 64-bit integer in the range
/// -9,223,372,036,854,775,808..9,223,372,036,854,775,807.
///
/// # Examples
/// ```
/// use fhir_core::primitives::Integer64;
///
/// let i = Integer64::new(42);
/// assert_eq!(i.as_i64(), 42);
///
/// let from_str = Integer64::try_from("9223372036854775807");
/// assert_eq!(from_str, Ok(Integer64::new(i64::MAX)));
///
/// let invalid = Integer64::try_from("007");
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deserialize)]
#[serde(try_from = "String")]
pub struct Integer64(i64);

impl Serialize for Integer64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // FHIR transmits `integer64` as a JSON string to avoid precision loss in
        // floating-point JSON libraries.
        serializer.serialize_str(&self.0.to_string())
    }
}

/// Granular errors encountered while validating a FHIR `integer64` string representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Integer64Error {
    /// The string is empty.
    Empty,
    /// The string contains a non-zero value with a disallowed leading zero (e.g. `"007"`).
    LeadingZero,
    /// The string contains characters that are not a valid integer representation
    /// (e.g. non-digit characters, a lone sign, or internal whitespace).
    InvalidFormat,
    /// The string parses as a valid integer format but its value falls outside the
    /// 64-bit signed integer range (-9,223,372,036,854,775,808..9,223,372,036,854,775,807).
    OutOfRange,
}

impl std::fmt::Display for Integer64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer64Error::Empty => write!(f, "integer64 string must not be empty"),
            Integer64Error::LeadingZero => {
                write!(f, "non-zero integer64 must not have a leading zero")
            }
            Integer64Error::InvalidFormat => write!(f, "not a valid integer64 representation"),
            Integer64Error::OutOfRange => write!(
                f,
                "value is out of range for a 64-bit signed integer \
                 (-9223372036854775808..9223372036854775807)"
            ),
        }
    }
}

impl std::error::Error for Integer64Error {}

impl Integer64 {
    /// The minimum value representable by a FHIR `integer64` (`i64::MIN`).
    pub const MIN: i64 = i64::MIN;

    /// The maximum value representable by a FHIR `integer64` (`i64::MAX`).
    pub const MAX: i64 = i64::MAX;

    /// Creates a new `Integer64` instance wrapping the given `i64` value.
    ///
    /// Infallible: every `i64` value is a valid FHIR `integer64`.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::primitives::Integer64;
    ///
    /// let i = Integer64::new(-7);
    /// assert_eq!(i.as_i64(), -7);
    /// ```
    #[inline]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the underlying value as a primitive `i64`.
    #[inline]
    pub const fn as_i64(&self) -> i64 {
        self.0
    }

    /// Returns a mutable reference to the underlying `i64` value.
    ///
    /// Every `i64` value is a valid FHIR `integer64`, so mutation through this reference
    /// cannot produce an invalid `Integer64`.
    #[inline]
    pub fn as_i64_mut(&mut self) -> &mut i64 {
        &mut self.0
    }

    /// Consumes the `Integer64` wrapper and returns the underlying primitive `i64`.
    #[inline]
    pub const fn into_inner(self) -> i64 {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `integer64` format.
    ///
    /// # Returns
    /// - `Ok(i64)` with the parsed value if the string matches the FHIR `integer64` regex
    ///   (`[0]|[-+]?[1-9][0-9]*`) and fits within the 64-bit signed integer range.
    /// - `Err(Integer64Error)` describing the exact failure reason otherwise.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::primitives::Integer64;
    ///
    /// assert_eq!(Integer64::validate("0"), Ok(0));
    /// assert_eq!(Integer64::validate("-42"), Ok(-42));
    /// assert!(Integer64::validate("007").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<i64, Integer64Error> {
        if value.is_empty() {
            return Err(Integer64Error::Empty);
        }

        let has_sign = matches!(value.as_bytes()[0], b'+' | b'-');
        let digits = if has_sign {
            value.get(1..).unwrap_or_default()
        } else {
            value
        };

        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(Integer64Error::InvalidFormat);
        }

        if digits == "0" {
            // The FHIR regex `[0]|[-+]?[1-9][0-9]*` only allows a bare `0`; a signed zero
            // (`+0`/`-0`) matches neither alternative and is therefore invalid.
            if has_sign {
                return Err(Integer64Error::InvalidFormat);
            }
        } else if digits.starts_with('0') {
            return Err(Integer64Error::LeadingZero);
        }

        value.parse::<i64>().map_err(|_| Integer64Error::OutOfRange)
    }

    /// Creates a new `Integer64` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses the FHIR string-format validation (leading zeros, sign placement, etc.).
    /// The caller is responsible for ensuring that the provided value is valid.
    #[inline]
    pub const fn new_unchecked(value: i64) -> Self {
        Self(value)
    }
}

impl From<i64> for Integer64 {
    #[inline]
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<Integer64> for i64 {
    #[inline]
    fn from(value: Integer64) -> Self {
        value.0
    }
}

impl TryFrom<&str> for Integer64 {
    type Error = TypeError;

    /// Parses an `Integer64` from a string slice, validating the FHIR `integer64` string format.
    ///
    /// # Errors
    /// Returns [`TypeError::InvalidValue`] if the input string is empty, malformed, contains a
    /// disallowed leading zero, or falls outside the 64-bit signed integer range.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value)
            .map(Self)
            .map_err(|e| TypeError::InvalidValue {
                r#type: "integer64".to_owned(),
                value: value.to_owned(),
                error: e.to_string(),
            })
    }
}

impl TryFrom<String> for Integer64 {
    type Error = TypeError;

    /// Parses an `Integer64` from an owned `String`.
    ///
    /// # Errors
    /// Returns [`TypeError::InvalidValue`] if the input string does not conform to the FHIR
    /// `integer64` format.
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Integer64 {
    type Err = TypeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl std::fmt::Display for Integer64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
