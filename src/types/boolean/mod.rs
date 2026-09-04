//! A module for the FHIR `boolean` primitive data type.
//!
//! This module provides a [`Boolean`] struct that encapsulates a boolean value (`true` or `false`)
//! according to the FHIR specification.
//!
//! # Overview
//! The FHIR `boolean` primitive represents a binary state of `true` or `false`.
//!
//! In FHIR:
//! - Value is either `true` or `false`.
//! - No intermediate, null-like, or numeric representations (e.g. `1` or `0`) are permitted as valid booleans.
//!
//! # Usage
//! To create a new [`Boolean`] instance:
//! - Use [`Boolean::new`] from a `bool`.
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse from a string slice (`"true"` or `"false"`).
//! - Use standard conversion traits like [`From<bool>`] or [`Into<bool>`].

use serde::{Deserialize, Serialize};

use crate::errors::r#type::TypeError;

#[cfg(test)]
mod test;

/// Represents a FHIR `boolean` primitive data type.
///
/// A binary value that is either `true` or `false`.
///
/// # Examples
/// ```
/// use fhir_core::types::Boolean;
///
/// let b = Boolean::new(true);
/// assert!(b.as_bool());
///
/// let from_str = Boolean::try_from("true");
/// assert_eq!(from_str, Ok(Boolean::new(true)));
///
/// let invalid = Boolean::try_from("yes");
/// assert!(invalid.is_err());
/// ```
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Boolean(bool);

impl Boolean {
    /// Creates a new `Boolean` instance wrapping the given boolean value.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Boolean;
    ///
    /// let b_true = Boolean::new(true);
    /// assert!(b_true.as_bool());
    ///
    /// let b_false = Boolean::new(false);
    /// assert!(!b_false.as_bool());
    /// ```
    #[inline]
    pub const fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the underlying boolean value as a primitive `bool`.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Boolean;
    ///
    /// let b = Boolean::new(true);
    /// assert_eq!(b.as_bool(), true);
    /// ```
    #[inline]
    pub const fn as_bool(&self) -> bool {
        self.0
    }

    /// Returns a static string slice representation (`"true"` or `"false"`).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Boolean;
    ///
    /// assert_eq!(Boolean::new(true).as_str(), "true");
    /// assert_eq!(Boolean::new(false).as_str(), "false");
    /// ```
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        if self.0 { "true" } else { "false" }
    }

    /// Consumes the `Boolean` wrapper and returns the underlying primitive `bool`.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Boolean;
    ///
    /// let b = Boolean::new(true);
    /// assert_eq!(b.into_inner(), true);
    /// ```
    #[inline]
    pub const fn into_inner(self) -> bool {
        self.0
    }
}

impl From<bool> for Boolean {
    #[inline]
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<Boolean> for bool {
    #[inline]
    fn from(value: Boolean) -> Self {
        value.0
    }
}

impl TryFrom<&str> for Boolean {
    type Error = TypeError;

    /// Parses a `Boolean` from a string slice. Only `"true"` and `"false"` are valid.
    ///
    /// # Errors
    /// Returns [`TypeError::InvalidValue`] if the input string is not `"true"` or `"false"`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "true" => Ok(Self(true)),
            "false" => Ok(Self(false)),
            _ => Err(TypeError::InvalidValue {
                r#type: "boolean".to_owned(),
                value: value.to_owned(),
                error: "expected 'true' or 'false'".to_owned(),
            }),
        }
    }
}

impl TryFrom<String> for Boolean {
    type Error = TypeError;

    /// Parses a `Boolean` from an owned `String`.
    ///
    /// # Errors
    /// Returns [`TypeError::InvalidValue`] if the input string is not `"true"` or `"false"`.
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Boolean {
    type Err = TypeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl std::fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
