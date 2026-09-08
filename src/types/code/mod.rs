//! A module for the FHIR `code` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#code)
//! - Regex: `[^\s]+( [^\s]+)*`
//! - "A code is restricted to a string which has at least one character and no leading
//!   or trailing whitespace, and where there is no whitespace other than single spaces
//!   in the contents."
//! - JSON encoding: a JSON string.
//! - Typically the value of a coded field bound to a `ValueSet` (e.g. `Patient.gender`).
//!
//! # Usage
//! To create a new [`Code`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`Code::new_unchecked`] when the input is already known to be valid.

use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `code` primitive data type.
///
/// A non-empty string with no leading/trailing whitespace and no whitespace other than
/// single spaces between non-whitespace tokens.
///
/// # Invariants
/// Any instance of `Code` is guaranteed to satisfy:
/// - Not empty.
/// - Does not start or end with whitespace.
/// - Contains no consecutive whitespace.
/// - Contains no whitespace character other than the literal space (`' '`) — no tabs,
///   newlines, or other Unicode whitespace.
///
/// # Examples
/// ```
/// use fhir_core::types::Code;
///
/// let valid = Code::new("active");
/// assert!(valid.is_ok());
///
/// let invalid = Code::new(" active");
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct Code(String);

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `code`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeError {
    /// The value is empty.
    Empty,
    /// The value starts with whitespace.
    LeadingWhitespace,
    /// The value ends with whitespace.
    TrailingWhitespace,
    /// Two or more consecutive whitespace characters were found between tokens.
    ConsecutiveWhitespace {
        /// The zero-based byte index of the second whitespace character in the run.
        index: usize,
    },
    /// A whitespace character other than a single literal space (`' '`) was encountered,
    /// e.g. a tab or newline.
    DisallowedWhitespaceCharacter {
        /// The disallowed whitespace character encountered.
        char: char,
        /// The zero-based byte index of the character.
        index: usize,
    },
}

impl std::fmt::Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeError::Empty => write!(f, "code must not be empty"),
            CodeError::LeadingWhitespace => write!(f, "code must not start with whitespace"),
            CodeError::TrailingWhitespace => write!(f, "code must not end with whitespace"),
            CodeError::ConsecutiveWhitespace { index } => {
                write!(f, "consecutive whitespace at byte index {index}")
            }
            CodeError::DisallowedWhitespaceCharacter { char, index } => {
                write!(
                    f,
                    "disallowed whitespace character {char:?} at byte index {index}; only a single literal space is permitted"
                )
            }
        }
    }
}

impl std::error::Error for CodeError {}

impl Code {
    /// Creates a new `Code` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input does not conform to the FHIR `code` specification.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Code;
    ///
    /// let code = Code::new("active").unwrap();
    /// assert_eq!(code.as_str(), "active");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying `code` value.
    ///
    /// `Code` intentionally exposes `as_str` rather than implementing `Deref<Target = str>`
    /// or `AsRef<str>` to prevent unwanted conversions that bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `Code` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `code` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string is non-empty, has no leading/trailing whitespace, and
    ///   contains no whitespace other than single spaces between tokens.
    /// - `Err(CodeError)` describing the exact failure reason and location if invalid.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Code;
    ///
    /// assert!(Code::validate("oral route").is_ok());
    /// assert!(Code::validate(" oral route").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<(), CodeError> {
        if value.is_empty() {
            return Err(CodeError::Empty);
        }

        let mut prev: Option<char> = None;

        for (byte_index, ch) in value.char_indices() {
            if ch == ' ' {
                match prev {
                    None => return Err(CodeError::LeadingWhitespace),
                    Some(' ') => {
                        return Err(CodeError::ConsecutiveWhitespace { index: byte_index });
                    }
                    _ => {}
                }
            } else if ch.is_whitespace() {
                return Err(CodeError::DisallowedWhitespaceCharacter {
                    char: ch,
                    index: byte_index,
                });
            }
            prev = Some(ch);
        }

        if prev == Some(' ') {
            return Err(CodeError::TrailingWhitespace);
        }

        Ok(())
    }

    /// Creates a new `Code` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `code` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Code;
    ///
    /// let code = Code::new_unchecked("trusted-code");
    /// assert_eq!(code.as_str(), "trusted-code");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for Code {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "code".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Code {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Code {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Code> for String {
    fn from(value: Code) -> Self {
        value.0
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
