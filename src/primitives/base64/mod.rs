//! A module for the FHIR `base64Binary` primitive data type.
//!
//! This module provides a `Base64Binary` struct that encapsulates a Base64-encoded string, ensuring
//! that any instance adheres to the Base64 specification. It includes validation logic, error
//! handling, and conversion traits for seamless integration with Rust's type system.
//!
//! # Overview
//! The `Base64Binary` type represents a stream of bytes encoded using standard Base64 (RFC 4648).
//! It guarantees that any instance is a valid Base64 string, with the following invariants:
//! - The length is a multiple of 4.
//! - Contains only valid Base64 characters (`A-Z`, `a-z`, `0-9`, `+`, `/`).
//! - Has at most 2 padding characters (`=`) at the end of the string.
//! - Contains no whitespace or unexpected padding.
//!
//! # Usage
//! To create a new `Base64Binary` instance, use the `Base64Binary::new` method, which validates the
//! input string. If the input is invalid, it returns a `FhirCoreError::Type` containing a
//! `TypeError::InvalidValue`. For cases where the input is known to be valid, you can use
//! `Base64Binary::new_unchecked`, which bypasses validation.

use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `base64Binary` primitive data type.
///
/// A stream of bytes encoded using standard Base64 (RFC 4648).
///
/// # Invariants
/// Any instance of `Base64Binary` is guaranteed to wrap a valid Base64 string:
/// - Its length is a multiple of 4.
/// - Contains only valid Base64 characters (`A-Z`, `a-z`, `0-9`, `+`, `/`).
/// - Has at most 2 padding characters (`=`) at the end of the string.
/// - Contains no whitespace or unexpected padding.
///
/// # Examples
/// ```
/// use fhir_core::primitives::Base64Binary;
///
/// let valid = Base64Binary::new("TWFu");
/// assert!(valid.is_ok());
///
/// let invalid = Base64Binary::new("TWF");
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct Base64Binary(String);

impl Serialize for Base64Binary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a Base64 string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Base64Error {
    /// The string length is not a multiple of 4.
    InvalidLength(usize),
    /// A character outside the standard Base64 alphabet was encountered.
    InvalidCharacter {
        /// The invalid character encountered.
        char: char,
        /// The zero-based byte index of the invalid character.
        index: usize,
    },
    /// A padding character (`=`) was encountered before the end of the string.
    UnexpectedPadding {
        /// The zero-based byte index where padding was unexpectedly found.
        index: usize,
    },
    /// More than 2 padding characters (`=`) were found at the end of the string.
    InvalidPadding {
        /// The total count of trailing padding characters.
        count: usize,
    },
    /// A padded block has fewer than 2 data characters (e.g. `A===`), which cannot represent a full byte.
    InsufficientDataBeforePadding,
}

impl std::fmt::Display for Base64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base64Error::InvalidLength(len) => write!(f, "length ({len}) is not a multiple of 4"),
            Base64Error::InvalidCharacter { char, index } => {
                write!(f, "invalid character '{char}' at index {index}")
            }
            Base64Error::UnexpectedPadding { index } => {
                write!(f, "unexpected padding character '=' at index {index}")
            }
            Base64Error::InvalidPadding { count } => write!(
                f,
                "invalid padding: found {count} '=' characters (max allowed is 2)"
            ),
            Base64Error::InsufficientDataBeforePadding => write!(
                f,
                "invalid padded block: requires at least 2 data characters before padding"
            ),
        }
    }
}

impl std::error::Error for Base64Error {}

impl Base64Binary {
    /// Creates a new `Base64Binary` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input does not conform to the Base64 specification.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::primitives::Base64Binary;
    ///
    /// // From a string slice (&str)
    /// let b64_from_str = Base64Binary::new("TQ==").unwrap();
    /// assert_eq!(b64_from_str.as_str(), "TQ==");
    ///
    /// // From an owned String
    /// let b64_from_string = Base64Binary::new(String::from("TWFu")).unwrap();
    /// assert_eq!(b64_from_string.as_str(), "TWFu");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying Base64 value.
    ///
    /// `Base64Binary` intentionally exposes `as_str` rather than implementing `Deref<Target = str>`
    /// or `AsRef<str>` to prevent unwanted conversions that bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `Base64Binary` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the standard Base64 format.
    ///
    /// # Returns
    /// - `Ok(())` if the string is valid Base64 or empty.
    /// - `Err(Base64Error)` describing the exact failure reason and location if invalid.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::primitives::Base64Binary;
    ///
    /// assert!(Base64Binary::validate("TWFu").is_ok());
    /// assert!(Base64Binary::validate("TWF").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<(), Base64Error> {
        let bytes = value.as_bytes();
        let len = bytes.len();

        if len == 0 {
            return Ok(());
        }

        // 1. Length check: Base64 strings with padding must be a multiple of 4
        if !len.is_multiple_of(4) {
            return Err(Base64Error::InvalidLength(len));
        }

        // 2. Count trailing padding '='
        let mut padding_count = 0;
        while padding_count < len && bytes[len - 1 - padding_count] == b'=' {
            padding_count += 1;
        }

        if padding_count > 2 {
            return Err(Base64Error::InvalidPadding {
                count: padding_count,
            });
        }

        let data_len = len - padding_count;

        // Padded 4-char block cannot have fewer than 2 data characters (e.g., "A===" is invalid)
        if padding_count > 0 && data_len % 4 < 2 {
            return Err(Base64Error::InsufficientDataBeforePadding);
        }

        // 3. Validate characters in the data portion
        for (index, &byte) in bytes[..data_len].iter().enumerate() {
            if byte == b'=' {
                return Err(Base64Error::UnexpectedPadding { index });
            }
            if !byte.is_ascii_alphanumeric() && byte != b'+' && byte != b'/' {
                return Err(Base64Error::InvalidCharacter {
                    char: byte as char,
                    index,
                });
            }
        }

        Ok(())
    }

    /// Creates a new `Base64Binary` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the Base64 format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::primitives::Base64Binary;
    ///
    /// let b64 = Base64Binary::new_unchecked("TWFu");
    /// assert_eq!(b64.as_str(), "TWFu");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for Base64Binary {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "base64Binary".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Base64Binary {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Base64Binary {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Base64Binary> for String {
    fn from(value: Base64Binary) -> Self {
        value.0
    }
}
