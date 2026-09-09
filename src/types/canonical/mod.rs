//! A module for the FHIR `canonical` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#canonical)
//! - Regex: `\S*`
//! - "A URI that refers to a resource by its canonical URL (resources with a `url`
//!   property). The `canonical` type differs from a `uri` in that it has special
//!   meaning in this specification, and in that it may have a version appended,
//!   separated by a vertical bar (`|`)."
//! - The optional `|version` suffix and `#fragment` are semantic conventions, not
//!   separate character classes — `|` and `#` are already permitted by the base `\S*`
//!   grammar, so no extra parsing is needed to accept them.
//! - "Unlike other URIs, canonical URLs are never relative — they are either absolute
//!   URIs, or fragment identifiers" is a semantic (not lexical/regex) constraint, not
//!   enforced here, matching [`Uri`](crate::types::Uri)/[`Url`](crate::types::Url):
//!   real validity is out of scope for this crate's primitive layer.
//! - `\S*` uses `*`, not `+`: the empty string is a valid `canonical`.
//! - JSON encoding: a JSON string.
//! - `canonical`, `uri`, and `url` are never substituted for each other per spec, so
//!   this is its own distinct type despite sharing `Uri`/`Url`'s validation rule.
//!
//! # Usage
//! To create a new [`Canonical`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`Canonical::new_unchecked`] when the input is already known to be valid.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `canonical` primitive data type.
///
/// Any string containing no whitespace characters (the empty string is valid),
/// optionally carrying a `|version` suffix or `#fragment`.
///
/// # Invariants
/// Any instance of `Canonical` is guaranteed to satisfy:
/// - Contains no whitespace characters.
///
/// # Examples
/// ```
/// use fhir_core::types::Canonical;
///
/// let valid = Canonical::new("http://hl7.org/fhir/StructureDefinition/Patient|5.0.0");
/// assert!(valid.is_ok());
///
/// let invalid = Canonical::new("http://example .org");
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
pub struct Canonical(String);

#[cfg(feature = "serde")]
impl Serialize for Canonical {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `canonical`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalError {
    /// A whitespace character was encountered, which `\S*` disallows.
    InvalidCharacter {
        /// The invalid whitespace character encountered.
        char: char,
        /// The zero-based byte index of the character.
        index: usize,
    },
}

impl std::fmt::Display for CanonicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanonicalError::InvalidCharacter { char, index } => {
                write!(
                    f,
                    "invalid whitespace character {char:?} at byte index {index}"
                )
            }
        }
    }
}

impl std::error::Error for CanonicalError {}

impl Canonical {
    /// Creates a new `Canonical` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input contains a whitespace character.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Canonical;
    ///
    /// let canonical = Canonical::new("http://hl7.org/fhir/StructureDefinition/Patient").unwrap();
    /// assert_eq!(canonical.as_str(), "http://hl7.org/fhir/StructureDefinition/Patient");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying `canonical` value.
    ///
    /// `Canonical` intentionally exposes `as_str` rather than implementing
    /// `Deref<Target = str>` or `AsRef<str>` to prevent unwanted conversions that
    /// bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `Canonical` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `canonical` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string contains no whitespace characters (the empty string is valid).
    /// - `Err(CanonicalError)` naming the offending whitespace character and its position otherwise.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Canonical;
    ///
    /// assert!(Canonical::validate("").is_ok());
    /// assert!(Canonical::validate("uri|1.0#frag").is_ok());
    /// assert!(Canonical::validate("http://example .org").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<(), CanonicalError> {
        if let Some((index, char)) = value.char_indices().find(|(_, c)| c.is_whitespace()) {
            return Err(CanonicalError::InvalidCharacter { char, index });
        }
        Ok(())
    }

    /// Creates a new `Canonical` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `canonical` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Canonical;
    ///
    /// let canonical = Canonical::new_unchecked("http://example.org|1.0");
    /// assert_eq!(canonical.as_str(), "http://example.org|1.0");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for Canonical {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "canonical".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Canonical {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Canonical {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Canonical> for String {
    fn from(value: Canonical) -> Self {
        value.0
    }
}

impl std::fmt::Display for Canonical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
