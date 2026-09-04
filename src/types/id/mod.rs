//! A module for the FHIR `id` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#id)
//! - Regex: `[A-Za-z0-9\-\.]{1,64}`
//! - Any combination of upper- or lower-case ASCII letters, numerals, `-`, and `.`, with a length
//!   limit of 64 characters.
//! - Case sensitive; UUIDs used as an `id` must be sent using lowercase letters.
//! - JSON encoding: a JSON string.
//! - More restrictive than [`FhirString`](crate::primitives::FhirString): no Unicode support, no
//!   whitespace, and a much shorter maximum length. Typically used for resource-local identifiers
//!   (e.g. the final path segment of `http://example.com/fhir/Patient/1234`), so it is validated
//!   independently rather than reusing `FhirString`'s validation.
//!
//! # Usage
//! To create a new [`Id`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`Id::new_unchecked`] when the input is already known to be valid.

use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `id` primitive data type.
///
/// A resource-local identifier: 1 to 64 ASCII letters, digits, hyphens, or periods.
///
/// # Invariants
/// Any instance of `Id` is guaranteed to satisfy:
/// - Not empty and does not exceed 64 characters.
/// - Contains only ASCII letters (`A-Z`, `a-z`), digits (`0-9`), hyphen (`-`), and period (`.`).
///
/// # Examples
/// ```
/// use fhir_core::types::Id;
///
/// let valid = Id::new("patient-1234");
/// assert!(valid.is_ok());
///
/// let invalid = Id::new("has a space");
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct Id(String);

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `id`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdError {
    /// The value is empty.
    Empty,
    /// The character count exceeds the maximum limit of 64 characters.
    ExceedsMaxLength {
        /// The actual character count found.
        count: usize,
        /// The maximum character count allowed.
        max: usize,
    },
    /// A character outside the allowed set (`A-Za-z0-9-.`) was encountered.
    InvalidCharacter {
        /// The invalid character encountered.
        char: char,
        /// The zero-based byte index of the invalid character.
        index: usize,
    },
}

impl std::fmt::Display for IdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdError::Empty => write!(f, "id must not be empty"),
            IdError::ExceedsMaxLength { count, max } => {
                write!(
                    f,
                    "character count ({count}) exceeds maximum allowed ({max})"
                )
            }
            IdError::InvalidCharacter { char, index } => {
                write!(f, "invalid character '{char}' at byte index {index}")
            }
        }
    }
}

impl std::error::Error for IdError {}

impl Id {
    /// The maximum allowed character count for a FHIR `id` (64 characters).
    pub const MAX_LENGTH: usize = 64;

    /// Creates a new `Id` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input does not conform to the FHIR `id` specification.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Id;
    ///
    /// let id_from_str = Id::new("patient-1234").unwrap();
    /// assert_eq!(id_from_str.as_str(), "patient-1234");
    ///
    /// let id_from_string = Id::new(String::from("Observation.1")).unwrap();
    /// assert_eq!(id_from_string.as_str(), "Observation.1");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying `id` value.
    ///
    /// `Id` intentionally exposes `as_str` rather than implementing `Deref<Target = str>`
    /// or `AsRef<str>` to prevent unwanted conversions that bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `Id` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `id` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string is 1 to 64 characters long and contains only ASCII letters,
    ///   digits, hyphens, and periods.
    /// - `Err(IdError)` describing the exact failure reason and location if invalid.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Id;
    ///
    /// assert!(Id::validate("abc-123.4").is_ok());
    /// assert!(Id::validate("has a space").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<(), IdError> {
        if value.is_empty() {
            return Err(IdError::Empty);
        }

        let mut char_count = 0;

        for (byte_index, ch) in value.char_indices() {
            char_count += 1;

            if !(ch.is_ascii_alphanumeric() || ch == '-' || ch == '.') {
                return Err(IdError::InvalidCharacter {
                    char: ch,
                    index: byte_index,
                });
            }

            if char_count > Self::MAX_LENGTH {
                let remaining = value[byte_index..].chars().count() - 1;
                return Err(IdError::ExceedsMaxLength {
                    count: char_count + remaining,
                    max: Self::MAX_LENGTH,
                });
            }
        }

        Ok(())
    }

    /// Creates a new `Id` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `id` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Id;
    ///
    /// let id = Id::new_unchecked("trusted-id");
    /// assert_eq!(id.as_str(), "trusted-id");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for Id {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "id".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Id {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Id {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Id> for String {
    fn from(value: Id) -> Self {
        value.0
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
