//! A module for the FHIR `string` primitive data type.
//!
//! This module provides a [`FhirString`] struct that encapsulates a sequence of Unicode characters,
//! ensuring that any instance adheres to the FHIR specification. It includes validation logic, error
//! handling, and conversion traits for seamless integration with Rust's type system.
//!
//! # Overview
//! The `string` primitive represents a sequence of Unicode characters (corresponding to XML's
//! `xs:string` and JSON String).
//!
//! Invariants according to the FHIR specification:
//! - SHALL NOT exceed 1,048,576 (`1024 * 1024`) characters in size. Because UTF-8 characters can be
//!   expressed with more than one byte, the byte length may be greater than 1MB.
//! - SHOULD NOT contain Unicode code points below 32, except for `\u{0009}` (horizontal tab `\t`),
//!   `\u{000A}` (line feed `\n`), and `\u{000D}` (carriage return `\r`).
//! - SHALL contain non-whitespace content (cannot be empty or consist solely of whitespace).
//!
//! # Usage
//! To create a new [`FhirString`] instance, use the [`FhirString::new`] method, which validates the
//! input string. If the input is invalid, it returns a [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type)
//! containing a [`TypeError::InvalidValue`]. For cases where the input is known to be valid, you can use
//! [`FhirString::new_unchecked`], which bypasses validation.

use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `string` primitive data type.
///
/// A sequence of Unicode characters with a maximum length of 1,048,576 characters, containing
/// non-whitespace content and no disallowed control characters.
///
/// # Invariants
/// Any instance of `FhirString` is guaranteed to satisfy:
/// - Not empty and contains at least one non-whitespace character.
/// - Length does not exceed 1,048,576 Unicode characters.
/// - Does not contain Unicode code points below 32, except `\t`, `\n`, and `\r`.
///
/// # Examples
/// ```
/// use fhir_core::types::FhirString;
///
/// let valid = FhirString::new("Hello, FHIR!");
/// assert!(valid.is_ok());
///
/// let empty = FhirString::new("");
/// assert!(empty.is_err());
///
/// let whitespace_only = FhirString::new("   \t\n  ");
/// assert!(whitespace_only.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct FhirString(String);

impl Serialize for FhirString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `string`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringError {
    /// The string is empty or contains only whitespace characters.
    EmptyOrWhitespaceOnly,
    /// The character count exceeds the maximum limit of 1,048,576 characters.
    ExceedsMaxLength {
        /// The actual character count found.
        count: usize,
        /// The maximum character count allowed.
        max: usize,
    },
    /// A disallowed Unicode control character (code point < 32, excluding `\t`, `\n`, `\r`) was encountered.
    DisallowedControlCharacter {
        /// The disallowed character encountered.
        char: char,
        /// The zero-based byte index where the disallowed character was found.
        index: usize,
    },
}

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringError::EmptyOrWhitespaceOnly => {
                write!(
                    f,
                    "string must contain non-whitespace content and cannot be empty"
                )
            }
            StringError::ExceedsMaxLength { count, max } => {
                write!(
                    f,
                    "character count ({count}) exceeds maximum allowed ({max})"
                )
            }
            StringError::DisallowedControlCharacter { char, index } => {
                write!(
                    f,
                    "disallowed control character {:?} (U+{:04X}) at byte index {index}",
                    char, *char as u32
                )
            }
        }
    }
}

impl std::error::Error for StringError {}

impl FhirString {
    /// The maximum allowed Unicode character count for a FHIR `string` (1,048,576 characters).
    pub const MAX_LENGTH: usize = 1024 * 1024;

    /// Creates a new `FhirString` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input does not conform to the FHIR `string` specification.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::FhirString;
    ///
    /// // From a string slice (&str)
    /// let s_from_str = FhirString::new("Patient Name").unwrap();
    /// assert_eq!(s_from_str.as_str(), "Patient Name");
    ///
    /// // From an owned String
    /// let s_from_string = FhirString::new(String::from("Observation Note")).unwrap();
    /// assert_eq!(s_from_string.as_str(), "Observation Note");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying string value.
    ///
    /// `FhirString` intentionally exposes `as_str` rather than implementing `Deref<Target = str>`
    /// or `AsRef<str>` to prevent unwanted conversions that bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `FhirString` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `string` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string contains non-whitespace content, does not exceed 1,048,576 characters,
    ///   and contains no disallowed control characters.
    /// - `Err(StringError)` describing the exact failure reason and location if invalid.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::FhirString;
    ///
    /// assert!(FhirString::validate("Valid content").is_ok());
    /// assert!(FhirString::validate("   ").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<(), StringError> {
        // 1. Must contain non-whitespace content (and not be empty)
        let mut has_non_whitespace = false;
        let mut char_count = 0;

        for (byte_index, ch) in value.char_indices() {
            char_count += 1;

            if !ch.is_whitespace() {
                has_non_whitespace = true;
            }

            // 2. Disallowed control characters: code point < 32, except \t (9), \n (10), \r (13)
            if (ch as u32) < 32 && ch != '\t' && ch != '\n' && ch != '\r' {
                return Err(StringError::DisallowedControlCharacter {
                    char: ch,
                    index: byte_index,
                });
            }

            // 3. Length check: max 1,048,576 Unicode characters
            if char_count > Self::MAX_LENGTH {
                let remaining = value[byte_index..].chars().count() - 1;
                return Err(StringError::ExceedsMaxLength {
                    count: char_count + remaining,
                    max: Self::MAX_LENGTH,
                });
            }
        }

        if !has_non_whitespace {
            return Err(StringError::EmptyOrWhitespaceOnly);
        }

        Ok(())
    }

    /// Creates a new `FhirString` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `string` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::FhirString;
    ///
    /// let s = FhirString::new_unchecked("Trusted Value");
    /// assert_eq!(s.as_str(), "Trusted Value");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for FhirString {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "string".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for FhirString {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value).map_err(|e| TypeError::InvalidValue {
            r#type: "string".to_owned(),
            value: value.clone(),
            error: e.to_string(),
        })?;
        Ok(Self(value))
    }
}

impl std::str::FromStr for FhirString {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<FhirString> for String {
    fn from(value: FhirString) -> Self {
        value.0
    }
}

impl std::fmt::Display for FhirString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
