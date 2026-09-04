//! A module for the FHIR `integer` primitive data type.
//!
//! This module provides an [`Integer`] struct that encapsulates a signed 32-bit integer value
//! according to the FHIR specification.
//!
//! # Overview
//! The FHIR `integer` primitive represents a signed 32-bit integer (corresponding to XML's
//! `xs:int` and JSON number without a decimal point).
//!
//! Invariants according to the FHIR specification:
//! - Value SHALL be in the range -2,147,483,648..2,147,483,647 (32-bit signed integer).
//! - String representation SHALL match the regex `[0]|[-+]?[1-9][0-9]*`:
//!   - `0` may stand alone.
//!   - Non-zero values MUST NOT have leading zeros.
//!   - An optional `+` or `-` sign is permitted for non-zero values (signed zeros like `+0` or
//!     `-0` are disallowed).
//!
//! # Usage
//! To create a new [`Integer`] instance:
//! - Use [`Integer::new`] from an `i32` (infallible; every `i32` is a valid FHIR integer).
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse from a string slice, which additionally
//!   validates the FHIR string format (rejecting leading zeros and malformed input).

use serde::{Deserialize, Serialize};

use crate::errors::r#type::TypeError;

#[cfg(test)]
mod test;

/// Represents a FHIR `integer` primitive data type.
///
/// A signed 32-bit integer in the range -2,147,483,648..2,147,483,647.
///
/// # Examples
/// ```
/// use fhir_core::types::Integer;
///
/// let i = Integer::new(42);
/// assert_eq!(i.as_i32(), 42);
///
/// let from_str = Integer::try_from("42");
/// assert_eq!(from_str, Ok(Integer::new(42)));
///
/// let invalid = Integer::try_from("007");
/// assert!(invalid.is_err());
/// ```
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Integer(i32);

/// Granular errors encountered while validating a FHIR `integer` string representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerError {
    /// The string is empty.
    Empty,
    /// The string contains a non-zero value with a disallowed leading zero (e.g. `"007"`).
    LeadingZero,
    /// The string contains characters that are not a valid integer representation
    /// (e.g. non-digit characters, a lone sign, or internal whitespace).
    InvalidFormat,
    /// The string parses as a valid integer format but its value falls outside the
    /// 32-bit signed integer range (-2,147,483,648..2,147,483,647).
    OutOfRange,
}

impl std::fmt::Display for IntegerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegerError::Empty => write!(f, "integer string must not be empty"),
            IntegerError::LeadingZero => {
                write!(f, "non-zero integer must not have a leading zero")
            }
            IntegerError::InvalidFormat => write!(f, "not a valid integer representation"),
            IntegerError::OutOfRange => write!(
                f,
                "value is out of range for a 32-bit signed integer (-2147483648..2147483647)"
            ),
        }
    }
}

impl std::error::Error for IntegerError {}

impl Integer {
    /// The minimum value representable by a FHIR `integer` (`i32::MIN`).
    pub const MIN: i32 = i32::MIN;

    /// The maximum value representable by a FHIR `integer` (`i32::MAX`).
    pub const MAX: i32 = i32::MAX;

    /// Creates a new `Integer` instance wrapping the given `i32` value.
    ///
    /// Infallible: every `i32` value is a valid FHIR `integer`.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Integer;
    ///
    /// let i = Integer::new(-7);
    /// assert_eq!(i.as_i32(), -7);
    /// ```
    #[inline]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Returns the underlying value as a primitive `i32`.
    #[inline]
    pub const fn as_i32(&self) -> i32 {
        self.0
    }

    /// Returns a mutable reference to the underlying `i32` value.
    ///
    /// Every `i32` value is a valid FHIR `integer`, so mutation through this reference
    /// cannot produce an invalid `Integer`.
    #[inline]
    pub fn as_i32_mut(&mut self) -> &mut i32 {
        &mut self.0
    }

    /// Consumes the `Integer` wrapper and returns the underlying primitive `i32`.
    #[inline]
    pub const fn into_inner(self) -> i32 {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `integer` format.
    ///
    /// # Returns
    /// - `Ok(i32)` with the parsed value if the string matches the FHIR `integer` regex
    ///   (`[0]|[-+]?[1-9][0-9]*`) and fits within the 32-bit signed integer range.
    /// - `Err(IntegerError)` describing the exact failure reason otherwise.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Integer;
    ///
    /// assert_eq!(Integer::validate("0"), Ok(0));
    /// assert_eq!(Integer::validate("-42"), Ok(-42));
    /// assert!(Integer::validate("007").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<i32, IntegerError> {
        if value.is_empty() {
            return Err(IntegerError::Empty);
        }

        let has_sign = matches!(value.as_bytes()[0], b'+' | b'-');
        let digits = if has_sign {
            value.get(1..).unwrap_or_default()
        } else {
            value
        };

        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(IntegerError::InvalidFormat);
        }

        if digits == "0" {
            // The FHIR regex `[0]|[-+]?[1-9][0-9]*` only allows a bare `0`; a signed zero
            // (`+0`/`-0`) matches neither alternative and is therefore invalid.
            if has_sign {
                return Err(IntegerError::InvalidFormat);
            }
        } else if digits.starts_with('0') {
            return Err(IntegerError::LeadingZero);
        }

        value.parse::<i32>().map_err(|_| IntegerError::OutOfRange)
    }

    /// Creates a new `Integer` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses the FHIR string-format validation (leading zeros, sign placement, etc.).
    /// The caller is responsible for ensuring that the provided value is valid.
    #[inline]
    pub const fn new_unchecked(value: i32) -> Self {
        Self(value)
    }
}

impl From<i32> for Integer {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<Integer> for i32 {
    #[inline]
    fn from(value: Integer) -> Self {
        value.0
    }
}

impl TryFrom<&str> for Integer {
    type Error = TypeError;

    /// Parses an `Integer` from a string slice, validating the FHIR `integer` string format.
    ///
    /// # Errors
    /// Returns [`TypeError::InvalidValue`] if the input string is empty, malformed, contains a
    /// disallowed leading zero, or falls outside the 32-bit signed integer range.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value)
            .map(Self)
            .map_err(|e| TypeError::InvalidValue {
                r#type: "integer".to_owned(),
                value: value.to_owned(),
                error: e.to_string(),
            })
    }
}

impl TryFrom<String> for Integer {
    type Error = TypeError;

    /// Parses an `Integer` from an owned `String`.
    ///
    /// # Errors
    /// Returns [`TypeError::InvalidValue`] if the input string does not conform to the FHIR
    /// `integer` format.
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Integer {
    type Err = TypeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
