//! A module for the FHIR `uri` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#uri)
//! - Regex: `\S*`
//! - "A Uniform Resource Identifier Reference (RFC 3986). Note: URIs are case
//!   sensitive. For UUID (urn:uuid:53fefa32-fcbb-4ff8-8a92-55ee120877b7) use all
//!   lowercase."
//! - The spec explicitly calls this regex "very permissive... informative, not
//!   normative" — real URI grammar (RFC 3986) is not enforced here, only the absence of
//!   whitespace, matching every other implementation's practical interpretation of `\S*`.
//! - `\S*` uses `*`, not `+`: the empty string is a valid `uri`.
//! - JSON encoding: a JSON string.
//!
//! # Usage
//! To create a new [`Uri`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`Uri::new_unchecked`] when the input is already known to be valid.

use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `uri` primitive data type.
///
/// Any string containing no whitespace characters (the empty string is valid).
///
/// # Invariants
/// Any instance of `Uri` is guaranteed to satisfy:
/// - Contains no whitespace characters.
///
/// # Examples
/// ```
/// use fhir_core::types::Uri;
///
/// let valid = Uri::new("http://example.org");
/// assert!(valid.is_ok());
///
/// let invalid = Uri::new("http://example .org");
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct Uri(String);

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `uri`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UriError {
    /// A whitespace character was encountered, which `\S*` disallows.
    InvalidCharacter {
        /// The invalid whitespace character encountered.
        char: char,
        /// The zero-based byte index of the character.
        index: usize,
    },
}

impl std::fmt::Display for UriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UriError::InvalidCharacter { char, index } => {
                write!(
                    f,
                    "invalid whitespace character {char:?} at byte index {index}"
                )
            }
        }
    }
}

impl std::error::Error for UriError {}

impl Uri {
    /// Creates a new `Uri` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input contains a whitespace character.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Uri;
    ///
    /// let uri = Uri::new("http://example.org").unwrap();
    /// assert_eq!(uri.as_str(), "http://example.org");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying `uri` value.
    ///
    /// `Uri` intentionally exposes `as_str` rather than implementing `Deref<Target = str>`
    /// or `AsRef<str>` to prevent unwanted conversions that bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `Uri` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `uri` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string contains no whitespace characters (the empty string is valid).
    /// - `Err(UriError)` naming the offending whitespace character and its position otherwise.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Uri;
    ///
    /// assert!(Uri::validate("").is_ok());
    /// assert!(Uri::validate("urn:uuid:123").is_ok());
    /// assert!(Uri::validate("has a space").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<(), UriError> {
        if let Some((index, char)) = value.char_indices().find(|(_, c)| c.is_whitespace()) {
            return Err(UriError::InvalidCharacter { char, index });
        }
        Ok(())
    }

    /// Creates a new `Uri` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `uri` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Uri;
    ///
    /// let uri = Uri::new_unchecked("http://example.org");
    /// assert_eq!(uri.as_str(), "http://example.org");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for Uri {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "uri".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Uri {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Uri {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Uri> for String {
    fn from(value: Uri) -> Self {
        value.0
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
