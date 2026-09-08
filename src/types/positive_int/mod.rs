//! A module for the FHIR `positiveInt` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#positiveInt)
//! - Regex: `[1-9][0-9]*`
//! - "Any positive `integer` in the range 1..2,147,483,647"
//! - No leading `+` sign, and no leading zeros are permitted.
//! - JSON encoding: a JSON number.
//! - Represented as `u32` rather than `i32`: unlike [`Integer`](crate::types::Integer),
//!   the valid domain (`1..=2,147,483,647`) is a strict subset of the wrapped native
//!   type's range, so both string parsing (`validate`) and numeric deserialization
//!   (`TryFrom<u32>`) must reject `0` and values above `2,147,483,647` even though `u32`
//!   itself can represent them.
//!
//! # Usage
//! To create a new [`PositiveInt`] instance:
//! - Use [`PositiveInt::new`] from a `u32` (fallible: `0` and values above the max are rejected).
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse from a string slice.
//! - Use [`PositiveInt::new_unchecked`] when the input is already known to be valid.

use serde::{Deserialize, Serialize};

use crate::errors::r#type::TypeError;

#[cfg(test)]
mod test;

/// Represents a FHIR `positiveInt` primitive data type.
///
/// A positive integer in the range `1..=2,147,483,647`.
///
/// # Examples
/// ```
/// use fhir_core::types::PositiveInt;
///
/// let p = PositiveInt::new(42).unwrap();
/// assert_eq!(p.as_u32(), 42);
///
/// assert!(PositiveInt::new(0).is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "u32")]
pub struct PositiveInt(u32);

impl Serialize for PositiveInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}

/// Granular errors encountered while validating a FHIR `positiveInt`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositiveIntError {
    /// The string is empty.
    Empty,
    /// The value is `0`, which is not positive.
    NotPositive,
    /// The string contains a value with a disallowed leading zero (e.g. `"007"`).
    LeadingZero,
    /// The string contains characters that are not a valid `positiveInt` representation
    /// (e.g. non-digit characters, a sign, or internal whitespace).
    InvalidFormat,
    /// The value exceeds the maximum of `2,147,483,647`.
    OutOfRange,
}

impl std::fmt::Display for PositiveIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PositiveIntError::Empty => write!(f, "positiveInt string must not be empty"),
            PositiveIntError::NotPositive => write!(f, "positiveInt must not be 0"),
            PositiveIntError::LeadingZero => {
                write!(f, "positiveInt must not have a leading zero")
            }
            PositiveIntError::InvalidFormat => write!(f, "not a valid positiveInt representation"),
            PositiveIntError::OutOfRange => {
                write!(f, "value exceeds the maximum of 2147483647")
            }
        }
    }
}

impl std::error::Error for PositiveIntError {}

impl PositiveInt {
    /// The minimum value representable by a FHIR `positiveInt`.
    pub const MIN: u32 = 1;

    /// The maximum value representable by a FHIR `positiveInt` (`i32::MAX` as `u32`).
    pub const MAX: u32 = 2_147_483_647;

    /// Creates a new `PositiveInt` instance wrapping the given `u32` value.
    ///
    /// # Errors
    /// Returns [`TypeError::InvalidValue`] if `value` is `0` or exceeds [`PositiveInt::MAX`].
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::PositiveInt;
    ///
    /// let p = PositiveInt::new(7).unwrap();
    /// assert_eq!(p.as_u32(), 7);
    /// ```
    pub fn new(value: u32) -> Result<Self, TypeError> {
        Self::try_from(value)
    }

    /// Returns the underlying value as a primitive `u32`.
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    /// Consumes the `PositiveInt` wrapper and returns the underlying primitive `u32`.
    #[inline]
    pub const fn into_inner(self) -> u32 {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `positiveInt` format.
    ///
    /// # Returns
    /// - `Ok(u32)` with the parsed value if the string matches the FHIR `positiveInt`
    ///   regex (`[1-9][0-9]*`) and falls within `1..=2,147,483,647`.
    /// - `Err(PositiveIntError)` describing the exact failure reason otherwise.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::PositiveInt;
    ///
    /// assert_eq!(PositiveInt::validate("42"), Ok(42));
    /// assert!(PositiveInt::validate("0").is_err());
    /// assert!(PositiveInt::validate("007").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<u32, PositiveIntError> {
        if value.is_empty() {
            return Err(PositiveIntError::Empty);
        }

        if !value.bytes().all(|b| b.is_ascii_digit()) {
            return Err(PositiveIntError::InvalidFormat);
        }

        if value == "0" {
            return Err(PositiveIntError::NotPositive);
        }

        if value.as_bytes()[0] == b'0' {
            return Err(PositiveIntError::LeadingZero);
        }

        let parsed: u32 = value.parse().map_err(|_| PositiveIntError::OutOfRange)?;
        if parsed > Self::MAX {
            return Err(PositiveIntError::OutOfRange);
        }

        Ok(parsed)
    }

    /// Creates a new `PositiveInt` instance without validating the input value.
    ///
    /// # Warning
    /// This bypasses validation. The caller is responsible for ensuring that `value` is
    /// in `1..=2,147,483,647`.
    #[inline]
    pub const fn new_unchecked(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<u32> for PositiveInt {
    type Error = TypeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 || value > Self::MAX {
            return Err(TypeError::InvalidValue {
                r#type: "positiveInt".to_owned(),
                value: value.to_string(),
                error: if value == 0 {
                    PositiveIntError::NotPositive.to_string()
                } else {
                    PositiveIntError::OutOfRange.to_string()
                },
            });
        }
        Ok(Self(value))
    }
}

impl From<PositiveInt> for u32 {
    #[inline]
    fn from(value: PositiveInt) -> Self {
        value.0
    }
}

impl TryFrom<&str> for PositiveInt {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value)
            .map(Self)
            .map_err(|e| TypeError::InvalidValue {
                r#type: "positiveInt".to_owned(),
                value: value.to_owned(),
                error: e.to_string(),
            })
    }
}

impl TryFrom<String> for PositiveInt {
    type Error = TypeError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for PositiveInt {
    type Err = TypeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl std::fmt::Display for PositiveInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
