//! A module for the FHIR `oid` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#oid)
//! - Regex: `urn:oid:[0-2](\.(0|[1-9][0-9]*))+`
//! - "An OID represented as a URI (RFC 3001); e.g. urn:oid:1.2.3.4.5"
//! - The `urn:oid:` prefix is mandatory.
//! - The root arc is a single digit `0`, `1`, or `2`.
//! - At least one further arc is required after the root; each arc is either a bare
//!   `0` or a digit `1`-`9` followed by any number of digits (no leading zeros on
//!   multi-digit arcs).
//! - JSON encoding: a JSON string.
//!
//! # Usage
//! To create a new [`Oid`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`Oid::new_unchecked`] when the input is already known to be valid.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `oid` primitive data type.
///
/// An OID represented as a URI: `urn:oid:` followed by a root arc (`0`, `1`, or `2`)
/// and one or more further dot-separated arcs.
///
/// # Invariants
/// Any instance of `Oid` is guaranteed to satisfy:
/// - Starts with the literal prefix `urn:oid:`.
/// - The root arc is exactly one of `0`, `1`, `2`.
/// - At least one further arc follows, each either `0` or a non-zero-leading digit run.
///
/// # Examples
/// ```
/// use fhir_core::types::Oid;
///
/// let valid = Oid::new("urn:oid:1.2.3.4.5");
/// assert!(valid.is_ok());
///
/// let invalid = Oid::new("1.2.3.4.5"); // missing "urn:oid:" prefix
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
pub struct Oid(String);

#[cfg(feature = "serde")]
impl Serialize for Oid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `oid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OidError {
    /// The value is empty.
    Empty,
    /// The value does not start with the literal prefix `urn:oid:`.
    MissingPrefix,
    /// The string ends immediately after the `urn:oid:` prefix, with no root arc digit.
    MissingRootArc,
    /// The root arc character is not `0`, `1`, or `2`.
    InvalidRootArc {
        /// The offending root arc character.
        found: char,
    },
    /// The root arc was valid but no further `.arc` segment follows it.
    MissingArc,
    /// A `.` was found but no digits followed it.
    InvalidArc {
        /// Always empty; kept as a struct field for symmetry with the other segment errors.
        found: String,
    },
    /// An arc has more than one digit and starts with `0` (e.g. `01`).
    LeadingZeroInArc {
        /// The offending arc segment.
        found: String,
    },
    /// An unexpected character was found where a `.` separator or end-of-string was expected.
    UnexpectedCharacter {
        /// The unexpected character.
        char: char,
        /// The zero-based byte index of the character.
        index: usize,
    },
}

impl std::fmt::Display for OidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OidError::Empty => write!(f, "oid must not be empty"),
            OidError::MissingPrefix => write!(f, "oid must start with 'urn:oid:'"),
            OidError::MissingRootArc => write!(f, "oid is missing a root arc after 'urn:oid:'"),
            OidError::InvalidRootArc { found } => {
                write!(f, "invalid root arc '{found}': must be '0', '1', or '2'")
            }
            OidError::MissingArc => {
                write!(f, "oid must have at least one arc after the root arc")
            }
            OidError::InvalidArc { .. } => {
                write!(f, "expected a digit after '.'")
            }
            OidError::LeadingZeroInArc { found } => {
                write!(f, "arc '{found}' must not have a leading zero")
            }
            OidError::UnexpectedCharacter { char, index } => {
                write!(f, "unexpected character '{char}' at byte index {index}")
            }
        }
    }
}

impl std::error::Error for OidError {}

impl Oid {
    /// The mandatory literal prefix of every FHIR `oid` value.
    pub const PREFIX: &'static str = "urn:oid:";

    /// Creates a new `Oid` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input does not conform to the FHIR `oid` specification.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Oid;
    ///
    /// let oid = Oid::new("urn:oid:1.2.3.4").unwrap();
    /// assert_eq!(oid.as_str(), "urn:oid:1.2.3.4");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying `oid` value.
    ///
    /// `Oid` intentionally exposes `as_str` rather than implementing `Deref<Target = str>`
    /// or `AsRef<str>` to prevent unwanted conversions that bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `Oid` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `oid` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string is `urn:oid:` followed by a root arc (`0`-`2`) and one
    ///   or more further dot-separated arcs.
    /// - `Err(OidError)` describing the exact failure reason if invalid.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Oid;
    ///
    /// assert!(Oid::validate("urn:oid:2.999.1").is_ok());
    /// assert!(Oid::validate("urn:oid:1.01").is_err()); // leading zero in arc
    /// ```
    pub fn validate(value: &str) -> Result<(), OidError> {
        if value.is_empty() {
            return Err(OidError::Empty);
        }

        let Some(rest) = value.strip_prefix(Self::PREFIX) else {
            return Err(OidError::MissingPrefix);
        };

        let mut chars = rest.chars();
        let root = chars.next().ok_or(OidError::MissingRootArc)?;
        if !matches!(root, '0'..='2') {
            return Err(OidError::InvalidRootArc { found: root });
        }

        let mut remaining = &rest[1..];
        if remaining.is_empty() {
            return Err(OidError::MissingArc);
        }

        let base_offset = Self::PREFIX.len() + 1;
        let mut consumed = 0usize;

        while !remaining.is_empty() {
            if !remaining.starts_with('.') {
                return Err(OidError::UnexpectedCharacter {
                    char: remaining.chars().next().expect("non-empty checked above"),
                    index: base_offset + consumed,
                });
            }
            remaining = &remaining[1..];
            consumed += 1;

            let digit_len = remaining.chars().take_while(char::is_ascii_digit).count();
            if digit_len == 0 {
                return Err(OidError::InvalidArc {
                    found: String::new(),
                });
            }
            let arc = &remaining[..digit_len];
            if arc.len() > 1 && arc.as_bytes()[0] == b'0' {
                return Err(OidError::LeadingZeroInArc {
                    found: arc.to_owned(),
                });
            }

            remaining = &remaining[digit_len..];
            consumed += digit_len;
        }

        Ok(())
    }

    /// Creates a new `Oid` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `oid` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Oid;
    ///
    /// let oid = Oid::new_unchecked("urn:oid:1.2.3");
    /// assert_eq!(oid.as_str(), "urn:oid:1.2.3");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for Oid {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "oid".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Oid {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Oid {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Oid> for String {
    fn from(value: Oid) -> Self {
        value.0
    }
}

impl std::fmt::Display for Oid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
