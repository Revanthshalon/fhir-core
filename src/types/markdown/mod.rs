//! A module for the FHIR `markdown` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#markdown)
//! - Regex: `^[\s\S]+$` (at least one character; the `+` here, unlike `uri`'s `\S*`,
//!   means the empty string is rejected)
//! - "A FHIR `string` that may contain markdown syntax for optional processing by a
//!   markdown presentation engine, in the GFM extension of CommonMark format."
//! - "Markdown is a string, and subject to the same rules (e.g. length limit, valid
//!   characters)" — same invariants as [`FhirString`](crate::types::FhirString): max
//!   1,048,576 characters, non-whitespace content required, and the same disallowed
//!   control-character set. Validated independently here rather than reusing
//!   `FhirString`, per this crate's convention (see `Id`'s module doc) of not coupling
//!   primitive modules to each other.
//! - JSON encoding: a JSON string.
//!
//! # Usage
//! To create a new [`Markdown`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`Markdown::new_unchecked`] when the input is already known to be valid.

use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `markdown` primitive data type.
///
/// A sequence of Unicode characters, optionally containing GFM CommonMark syntax, with
/// a maximum length of 1,048,576 characters, containing non-whitespace content and no
/// disallowed control characters.
///
/// # Invariants
/// Any instance of `Markdown` is guaranteed to satisfy:
/// - Not empty and contains at least one non-whitespace character.
/// - Length does not exceed 1,048,576 Unicode characters.
/// - Does not contain Unicode code points below 32, except `\t`, `\n`, and `\r`.
///
/// # Examples
/// ```
/// use fhir_core::types::Markdown;
///
/// let valid = Markdown::new("# Header\n\n**bold**");
/// assert!(valid.is_ok());
///
/// let empty = Markdown::new("");
/// assert!(empty.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct Markdown(String);

impl Serialize for Markdown {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `markdown`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkdownError {
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

impl std::fmt::Display for MarkdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkdownError::EmptyOrWhitespaceOnly => {
                write!(
                    f,
                    "markdown must contain non-whitespace content and cannot be empty"
                )
            }
            MarkdownError::ExceedsMaxLength { count, max } => {
                write!(
                    f,
                    "character count ({count}) exceeds maximum allowed ({max})"
                )
            }
            MarkdownError::DisallowedControlCharacter { char, index } => {
                write!(
                    f,
                    "disallowed control character {:?} (U+{:04X}) at byte index {index}",
                    char, *char as u32
                )
            }
        }
    }
}

impl std::error::Error for MarkdownError {}

impl Markdown {
    /// The maximum allowed Unicode character count for a FHIR `markdown` (1,048,576 characters).
    pub const MAX_LENGTH: usize = 1024 * 1024;

    /// Creates a new `Markdown` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input does not conform to the FHIR `markdown` specification.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Markdown;
    ///
    /// let md = Markdown::new("Hello, *world*!").unwrap();
    /// assert_eq!(md.as_str(), "Hello, *world*!");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying `markdown` value.
    ///
    /// `Markdown` intentionally exposes `as_str` rather than implementing
    /// `Deref<Target = str>` or `AsRef<str>` to prevent unwanted conversions that
    /// bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `Markdown` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `markdown` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string contains non-whitespace content, does not exceed
    ///   1,048,576 characters, and contains no disallowed control characters.
    /// - `Err(MarkdownError)` describing the exact failure reason and location if invalid.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Markdown;
    ///
    /// assert!(Markdown::validate("**bold**").is_ok());
    /// assert!(Markdown::validate("   ").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<(), MarkdownError> {
        let mut has_non_whitespace = false;
        let mut char_count = 0;

        for (byte_index, ch) in value.char_indices() {
            char_count += 1;

            if !ch.is_whitespace() {
                has_non_whitespace = true;
            }

            if (ch as u32) < 32 && ch != '\t' && ch != '\n' && ch != '\r' {
                return Err(MarkdownError::DisallowedControlCharacter {
                    char: ch,
                    index: byte_index,
                });
            }

            if char_count > Self::MAX_LENGTH {
                let remaining = value[byte_index..].chars().count() - 1;
                return Err(MarkdownError::ExceedsMaxLength {
                    count: char_count + remaining,
                    max: Self::MAX_LENGTH,
                });
            }
        }

        if !has_non_whitespace {
            return Err(MarkdownError::EmptyOrWhitespaceOnly);
        }

        Ok(())
    }

    /// Creates a new `Markdown` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `markdown` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Markdown;
    ///
    /// let md = Markdown::new_unchecked("Trusted *value*");
    /// assert_eq!(md.as_str(), "Trusted *value*");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for Markdown {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "markdown".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Markdown {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value).map_err(|e| TypeError::InvalidValue {
            r#type: "markdown".to_owned(),
            value: value.clone(),
            error: e.to_string(),
        })?;
        Ok(Self(value))
    }
}

impl std::str::FromStr for Markdown {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Markdown> for String {
    fn from(value: Markdown) -> Self {
        value.0
    }
}

impl std::fmt::Display for Markdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
