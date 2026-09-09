//! A module for the FHIR `unsignedInt` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#unsignedInt)
//! - Regex: `[0]|([1-9][0-9]*)`
//! - "Any non-negative `integer` in the range 0..2,147,483,647"
//! - No leading `+` sign, and no leading zeros are permitted (other than the bare
//!   value `"0"` itself).
//! - JSON encoding: a JSON number.
//! - Represented as `u32` rather than `i32`: like
//!   [`PositiveInt`](crate::types::PositiveInt), the valid domain
//!   (`0..=2,147,483,647`) is a strict subset of the wrapped native type's range, so
//!   both string parsing (`validate`) and numeric deserialization (`TryFrom<u32>`) must
//!   reject values above `2,147,483,647` even though `u32` itself can represent them.
//!
//! # Usage
//! To create a new [`UnsignedInt`] instance:
//! - Use [`UnsignedInt::new`] from a `u32` (fallible: values above the max are rejected).
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse from a string slice.
//! - Use [`UnsignedInt::new_unchecked`] when the input is already known to be valid.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::errors::r#type::TypeError;

#[cfg(test)]
mod test;

/// Represents a FHIR `unsignedInt` primitive data type.
///
/// A non-negative integer in the range `0..=2,147,483,647`.
///
/// # Examples
/// ```
/// use fhir_core::types::UnsignedInt;
///
/// let u = UnsignedInt::new(0).unwrap();
/// assert_eq!(u.as_u32(), 0);
///
/// assert!(UnsignedInt::new(3_000_000_000).is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u32"))]
pub struct UnsignedInt(u32);

#[cfg(feature = "serde")]
impl Serialize for UnsignedInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}

/// Granular errors encountered while validating a FHIR `unsignedInt`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnsignedIntError {
    /// The string is empty.
    Empty,
    /// The string contains a non-zero value with a disallowed leading zero (e.g. `"007"`).
    LeadingZero,
    /// The string contains characters that are not a valid `unsignedInt` representation
    /// (e.g. non-digit characters, a sign, or internal whitespace).
    InvalidFormat,
    /// The value exceeds the maximum of `2,147,483,647`.
    OutOfRange,
}

impl std::fmt::Display for UnsignedIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnsignedIntError::Empty => write!(f, "unsignedInt string must not be empty"),
            UnsignedIntError::LeadingZero => {
                write!(f, "non-zero unsignedInt must not have a leading zero")
            }
            UnsignedIntError::InvalidFormat => {
                write!(f, "not a valid unsignedInt representation")
            }
            UnsignedIntError::OutOfRange => {
                write!(f, "value exceeds the maximum of 2147483647")
            }
        }
    }
}

impl std::error::Error for UnsignedIntError {}

impl UnsignedInt {
    /// The minimum value representable by a FHIR `unsignedInt`.
    pub const MIN: u32 = 0;

    /// The maximum value representable by a FHIR `unsignedInt` (`i32::MAX` as `u32`).
    pub const MAX: u32 = 2_147_483_647;

    /// Creates a new `UnsignedInt` instance wrapping the given `u32` value.
    ///
    /// # Errors
    /// Returns [`TypeError::InvalidValue`] if `value` exceeds [`UnsignedInt::MAX`].
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::UnsignedInt;
    ///
    /// let u = UnsignedInt::new(7).unwrap();
    /// assert_eq!(u.as_u32(), 7);
    /// ```
    pub fn new(value: u32) -> Result<Self, TypeError> {
        Self::try_from(value)
    }

    /// Returns the underlying value as a primitive `u32`.
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    /// Consumes the `UnsignedInt` wrapper and returns the underlying primitive `u32`.
    #[inline]
    pub const fn into_inner(self) -> u32 {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `unsignedInt` format.
    ///
    /// # Returns
    /// - `Ok(u32)` with the parsed value if the string matches the FHIR `unsignedInt`
    ///   regex (`[0]|([1-9][0-9]*)`) and falls within `0..=2,147,483,647`.
    /// - `Err(UnsignedIntError)` describing the exact failure reason otherwise.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::UnsignedInt;
    ///
    /// assert_eq!(UnsignedInt::validate("0"), Ok(0));
    /// assert_eq!(UnsignedInt::validate("42"), Ok(42));
    /// assert!(UnsignedInt::validate("007").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<u32, UnsignedIntError> {
        if value.is_empty() {
            return Err(UnsignedIntError::Empty);
        }

        if !value.bytes().all(|b| b.is_ascii_digit()) {
            return Err(UnsignedIntError::InvalidFormat);
        }

        if value != "0" && value.as_bytes()[0] == b'0' {
            return Err(UnsignedIntError::LeadingZero);
        }

        let parsed: u32 = value.parse().map_err(|_| UnsignedIntError::OutOfRange)?;
        if parsed > Self::MAX {
            return Err(UnsignedIntError::OutOfRange);
        }

        Ok(parsed)
    }

    /// Creates a new `UnsignedInt` instance without validating the input value.
    ///
    /// # Warning
    /// This bypasses validation. The caller is responsible for ensuring that `value` is
    /// in `0..=2,147,483,647`.
    #[inline]
    pub const fn new_unchecked(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<u32> for UnsignedInt {
    type Error = TypeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > Self::MAX {
            return Err(TypeError::InvalidValue {
                r#type: "unsignedInt".to_owned(),
                value: value.to_string(),
                error: UnsignedIntError::OutOfRange.to_string(),
            });
        }
        Ok(Self(value))
    }
}

impl From<UnsignedInt> for u32 {
    #[inline]
    fn from(value: UnsignedInt) -> Self {
        value.0
    }
}

impl TryFrom<&str> for UnsignedInt {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value)
            .map(Self)
            .map_err(|e| TypeError::InvalidValue {
                r#type: "unsignedInt".to_owned(),
                value: value.to_owned(),
                error: e.to_string(),
            })
    }
}

impl TryFrom<String> for UnsignedInt {
    type Error = TypeError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for UnsignedInt {
    type Err = TypeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl std::fmt::Display for UnsignedInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
