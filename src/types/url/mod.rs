//! A module for the FHIR `url` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#url)
//! - Regex: `\S*`
//! - "A Uniform Resource Locator (RFC 1738). Note URLs are accessed directly using the
//!   specified protocol. Common URL protocols are http{s}:, ftp:, mailto: and mllp:,
//!   though many others are defined."
//! - "Although the `url` and `canonical` are specializations of `uri`, they are never
//!   substituted for each other" — `url` is its own FHIR type (structurally identical
//!   validation to [`Uri`](crate::types::Uri) today, since the regex is the same
//!   permissive `\S*`), kept separate so a `url`-typed field can't silently accept a
//!   `Uri`.
//! - `\S*` uses `*`, not `+`: the empty string is a valid `url`.
//! - JSON encoding: a JSON string.
//!
//! # Usage
//! To create a new [`Url`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`Url::new_unchecked`] when the input is already known to be valid.

use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `url` primitive data type.
///
/// Any string containing no whitespace characters (the empty string is valid).
///
/// # Invariants
/// Any instance of `Url` is guaranteed to satisfy:
/// - Contains no whitespace characters.
///
/// # Examples
/// ```
/// use fhir_core::types::Url;
///
/// let valid = Url::new("https://example.org/index.html");
/// assert!(valid.is_ok());
///
/// let invalid = Url::new("https://example .org");
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct Url(String);

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `url`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UrlError {
    /// A whitespace character was encountered, which `\S*` disallows.
    InvalidCharacter {
        /// The invalid whitespace character encountered.
        char: char,
        /// The zero-based byte index of the character.
        index: usize,
    },
}

impl std::fmt::Display for UrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlError::InvalidCharacter { char, index } => {
                write!(
                    f,
                    "invalid whitespace character {char:?} at byte index {index}"
                )
            }
        }
    }
}

impl std::error::Error for UrlError {}

impl Url {
    /// Creates a new `Url` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input contains a whitespace character.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Url;
    ///
    /// let url = Url::new("https://example.org").unwrap();
    /// assert_eq!(url.as_str(), "https://example.org");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying `url` value.
    ///
    /// `Url` intentionally exposes `as_str` rather than implementing `Deref<Target = str>`
    /// or `AsRef<str>` to prevent unwanted conversions that bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `Url` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `url` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string contains no whitespace characters (the empty string is valid).
    /// - `Err(UrlError)` naming the offending whitespace character and its position otherwise.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Url;
    ///
    /// assert!(Url::validate("").is_ok());
    /// assert!(Url::validate("mailto:someone@example.org").is_ok());
    /// assert!(Url::validate("has a space").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<(), UrlError> {
        if let Some((index, char)) = value.char_indices().find(|(_, c)| c.is_whitespace()) {
            return Err(UrlError::InvalidCharacter { char, index });
        }
        Ok(())
    }

    /// Creates a new `Url` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `url` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Url;
    ///
    /// let url = Url::new_unchecked("https://example.org");
    /// assert_eq!(url.as_str(), "https://example.org");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for Url {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "url".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Url {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Url {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Url> for String {
    fn from(value: Url) -> Self {
        value.0
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
