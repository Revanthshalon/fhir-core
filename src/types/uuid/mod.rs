//! A module for the FHIR `uuid` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#uuid)
//! - Regex: `urn:uuid:[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}`
//! - "A UUID (aka GUID) represented as a URI (RFC 4122); e.g.
//!   urn:uuid:c757873d-ec9a-4326-a141-556f43239520"
//! - The `urn:uuid:` prefix is mandatory.
//! - "UUIDs SHALL be sent using lowercase letters" — the regex uses `[0-9a-f]`, not
//!   `[0-9a-fA-F]`, so uppercase hex digits are rejected rather than normalized.
//! - JSON encoding: a JSON string.
//!
//! # Usage
//! To create a new [`Uuid`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`Uuid::new_unchecked`] when the input is already known to be valid.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `uuid` primitive data type.
///
/// A UUID represented as a URI: `urn:uuid:` followed by the canonical
/// `8-4-4-4-12` lowercase-hex, hyphen-separated form.
///
/// # Invariants
/// Any instance of `Uuid` is guaranteed to satisfy:
/// - Starts with the literal prefix `urn:uuid:`.
/// - Followed by exactly 32 lowercase hex digits grouped `8-4-4-4-12` with hyphen
///   separators at the correct positions.
///
/// # Examples
/// ```
/// use fhir_core::types::Uuid;
///
/// let valid = Uuid::new("urn:uuid:c757873d-ec9a-4326-a141-556f43239520");
/// assert!(valid.is_ok());
///
/// let invalid = Uuid::new("urn:uuid:C757873D-EC9A-4326-A141-556F43239520"); // uppercase
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
pub struct Uuid(String);

#[cfg(feature = "serde")]
impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `uuid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UuidError {
    /// The value is empty.
    Empty,
    /// The value does not start with the literal prefix `urn:uuid:`.
    MissingPrefix,
    /// The portion after `urn:uuid:` is not exactly 36 characters (`8-4-4-4-12` hex
    /// groups plus 4 hyphens).
    InvalidLength {
        /// The actual character count found after the prefix.
        found: usize,
    },
    /// A character outside the allowed set (lowercase hex digits and hyphens at the
    /// correct group boundaries) was encountered.
    InvalidCharacter {
        /// The invalid character encountered.
        char: char,
        /// The zero-based byte index of the character within the whole value.
        index: usize,
    },
}

impl std::fmt::Display for UuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UuidError::Empty => write!(f, "uuid must not be empty"),
            UuidError::MissingPrefix => write!(f, "uuid must start with 'urn:uuid:'"),
            UuidError::InvalidLength { found } => {
                write!(
                    f,
                    "expected 36 characters after 'urn:uuid:' (8-4-4-4-12 hex groups), found {found}"
                )
            }
            UuidError::InvalidCharacter { char, index } => {
                write!(f, "invalid character '{char}' at byte index {index}")
            }
        }
    }
}

impl std::error::Error for UuidError {}

impl Uuid {
    /// The mandatory literal prefix of every FHIR `uuid` value.
    pub const PREFIX: &'static str = "urn:uuid:";

    /// The lengths of the 5 hyphen-separated hex groups in canonical UUID form.
    const GROUP_LENGTHS: [usize; 5] = [8, 4, 4, 4, 12];

    /// Creates a new `Uuid` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input does not conform to the FHIR `uuid` specification.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Uuid;
    ///
    /// let uuid = Uuid::new("urn:uuid:c757873d-ec9a-4326-a141-556f43239520").unwrap();
    /// assert_eq!(uuid.as_str(), "urn:uuid:c757873d-ec9a-4326-a141-556f43239520");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying `uuid` value.
    ///
    /// `Uuid` intentionally exposes `as_str` rather than implementing `Deref<Target = str>`
    /// or `AsRef<str>` to prevent unwanted conversions that bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `Uuid` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `uuid` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string is `urn:uuid:` followed by the canonical `8-4-4-4-12`
    ///   lowercase-hex UUID form.
    /// - `Err(UuidError)` describing the exact failure reason if invalid.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Uuid;
    ///
    /// assert!(Uuid::validate("urn:uuid:c757873d-ec9a-4326-a141-556f43239520").is_ok());
    /// assert!(Uuid::validate("urn:uuid:C757873D-EC9A-4326-A141-556F43239520").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<(), UuidError> {
        if value.is_empty() {
            return Err(UuidError::Empty);
        }

        let Some(rest) = value.strip_prefix(Self::PREFIX) else {
            return Err(UuidError::MissingPrefix);
        };

        if rest.len() != 36 {
            return Err(UuidError::InvalidLength {
                found: rest.chars().count(),
            });
        }

        let base_offset = Self::PREFIX.len();
        let mut idx = 0usize;

        for (group_idx, &len) in Self::GROUP_LENGTHS.iter().enumerate() {
            let group = &rest[idx..idx + len];
            for (offset, ch) in group.char_indices() {
                if !(ch.is_ascii_digit() || ('a'..='f').contains(&ch)) {
                    return Err(UuidError::InvalidCharacter {
                        char: ch,
                        index: base_offset + idx + offset,
                    });
                }
            }
            idx += len;

            if group_idx < Self::GROUP_LENGTHS.len() - 1 {
                if rest.as_bytes()[idx] != b'-' {
                    return Err(UuidError::InvalidCharacter {
                        char: rest[idx..].chars().next().expect("length checked above"),
                        index: base_offset + idx,
                    });
                }
                idx += 1;
            }
        }

        Ok(())
    }

    /// Creates a new `Uuid` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `uuid` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Uuid;
    ///
    /// let uuid = Uuid::new_unchecked("urn:uuid:c757873d-ec9a-4326-a141-556f43239520");
    /// assert_eq!(uuid.as_str(), "urn:uuid:c757873d-ec9a-4326-a141-556f43239520");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for Uuid {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "uuid".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Uuid {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Uuid {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Uuid> for String {
    fn from(value: Uuid) -> Self {
        value.0
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
