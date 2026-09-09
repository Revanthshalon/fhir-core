//! A module for the FHIR `time` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#time)
//! - Regex: `([01][0-9]|2[0-3]):[0-5][0-9]:([0-5][0-9]|60)(\.[0-9]{1,9})?`
//! - "A time during the day, in the format hh:mm:ss. There is no date specified.
//!   Seconds must be provided due to schema type constraints but may be zero-filled and
//!   may be ignored at receiver discretion. The time '24:00' SHALL NOT be used. A
//!   timezone offset SHALL NOT be present."
//! - "Leap Seconds are allowed" — seconds may be `60`.
//! - Fractional seconds: `\.[0-9]{1,9}`, i.e. 1 to 9 digits, optional.
//! - JSON encoding: a JSON string.
//!
//! # Usage
//! To create a new [`Time`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`Time::new_unchecked`] when the input is already known to be valid.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `time` primitive data type.
///
/// A time-of-day with no date and no timezone: `hh:mm:ss[.sss]`.
///
/// # Invariants
/// Any instance of `Time` is guaranteed to satisfy:
/// - Hour `00`-`23` (`24:00` is explicitly forbidden by the spec).
/// - Minute `00`-`59`.
/// - Second `00`-`60` (leap second permitted).
/// - Optional fractional seconds of 1-9 digits.
/// - No timezone offset (forbidden by the spec).
///
/// # Examples
/// ```
/// use fhir_core::types::Time;
///
/// let valid = Time::new("14:30:00.123");
/// assert!(valid.is_ok());
///
/// let invalid = Time::new("24:00:00"); // midnight-as-24:00 is forbidden
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
pub struct Time(String);

#[cfg(feature = "serde")]
impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `time`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeError {
    /// The value is empty.
    Empty,
    /// The value was shorter than the required `hh:mm:ss`.
    Incomplete {
        /// The offending remainder of the string.
        found: String,
    },
    /// The hour segment is not exactly 2 ASCII digits.
    InvalidHour {
        /// The offending hour segment.
        found: String,
    },
    /// The hour segment was 2 digits but not in `00..=23` (`24` is explicitly forbidden).
    HourOutOfRange {
        /// The out-of-range hour value.
        hour: u32,
    },
    /// The minute segment is not exactly 2 ASCII digits.
    InvalidMinute {
        /// The offending minute segment.
        found: String,
    },
    /// The minute segment was 2 digits but not in `00..=59`.
    MinuteOutOfRange {
        /// The out-of-range minute value.
        minute: u32,
    },
    /// The second segment is not exactly 2 ASCII digits.
    InvalidSecond {
        /// The offending second segment.
        found: String,
    },
    /// The second segment was 2 digits but not in `00..=60` (60 permitted for leap seconds).
    SecondOutOfRange {
        /// The out-of-range second value.
        second: u32,
    },
    /// The fractional-seconds segment after `.` was empty or had more than 9 digits.
    InvalidFractionalSeconds {
        /// The offending fractional-seconds segment.
        found: String,
    },
    /// A timezone offset (or other trailing content) was present; `time` forbids it.
    UnexpectedCharacter {
        /// The unexpected character.
        char: char,
        /// The zero-based byte index of the character.
        index: usize,
    },
}

impl std::fmt::Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeError::Empty => write!(f, "time must not be empty"),
            TimeError::Incomplete { found } => {
                write!(f, "incomplete time '{found}': expected hh:mm:ss")
            }
            TimeError::InvalidHour { found } => {
                write!(f, "invalid hour segment '{found}': must be 2 digits")
            }
            TimeError::HourOutOfRange { hour } => {
                write!(
                    f,
                    "hour {hour} out of range: must be 00-23 ('24:00' is forbidden)"
                )
            }
            TimeError::InvalidMinute { found } => {
                write!(f, "invalid minute segment '{found}': must be 2 digits")
            }
            TimeError::MinuteOutOfRange { minute } => {
                write!(f, "minute {minute} out of range: must be 00-59")
            }
            TimeError::InvalidSecond { found } => {
                write!(f, "invalid second segment '{found}': must be 2 digits")
            }
            TimeError::SecondOutOfRange { second } => {
                write!(
                    f,
                    "second {second} out of range: must be 00-60 (60 permitted for leap seconds)"
                )
            }
            TimeError::InvalidFractionalSeconds { found } => {
                write!(
                    f,
                    "invalid fractional seconds '{found}': must be 1-9 digits"
                )
            }
            TimeError::UnexpectedCharacter { char, index } => {
                write!(
                    f,
                    "unexpected character '{char}' at byte index {index}: a timezone offset is not permitted on 'time'"
                )
            }
        }
    }
}

impl std::error::Error for TimeError {}

impl Time {
    /// Creates a new `Time` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input does not conform to the FHIR `time` specification.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Time;
    ///
    /// let time = Time::new("14:30:00").unwrap();
    /// assert_eq!(time.as_str(), "14:30:00");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying `time` value.
    ///
    /// `Time` intentionally exposes `as_str` rather than implementing `Deref<Target = str>`
    /// or `AsRef<str>` to prevent unwanted conversions that bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `Time` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `time` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string is `hh:mm:ss[.sss]` with no date and no timezone.
    /// - `Err(TimeError)` describing the exact failure reason if invalid.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Time;
    ///
    /// assert!(Time::validate("23:59:60").is_ok()); // leap second
    /// assert!(Time::validate("24:00:00").is_err()); // forbidden
    /// ```
    pub fn validate(value: &str) -> Result<(), TimeError> {
        if value.is_empty() {
            return Err(TimeError::Empty);
        }
        if value.len() < 8 {
            return Err(TimeError::Incomplete {
                found: value.to_owned(),
            });
        }

        if !value.as_bytes()[0..2].iter().all(u8::is_ascii_digit) {
            return Err(TimeError::InvalidHour {
                found: value[0..2].to_owned(),
            });
        }
        let hour: u32 = value[0..2].parse().expect("checked all-ASCII-digit above");
        if hour > 23 {
            return Err(TimeError::HourOutOfRange { hour });
        }
        if value.as_bytes()[2] != b':' {
            return Err(TimeError::UnexpectedCharacter {
                char: value[2..].chars().next().expect("length checked above"),
                index: 2,
            });
        }

        if !value.as_bytes()[3..5].iter().all(u8::is_ascii_digit) {
            return Err(TimeError::InvalidMinute {
                found: value[3..5].to_owned(),
            });
        }
        let minute: u32 = value[3..5].parse().expect("checked all-ASCII-digit above");
        if minute > 59 {
            return Err(TimeError::MinuteOutOfRange { minute });
        }
        if value.as_bytes()[5] != b':' {
            return Err(TimeError::UnexpectedCharacter {
                char: value[5..].chars().next().expect("length checked above"),
                index: 5,
            });
        }

        if !value.as_bytes()[6..8].iter().all(u8::is_ascii_digit) {
            return Err(TimeError::InvalidSecond {
                found: value[6..8].to_owned(),
            });
        }
        let second: u32 = value[6..8].parse().expect("checked all-ASCII-digit above");
        if second > 60 {
            return Err(TimeError::SecondOutOfRange { second });
        }

        let mut cursor = &value[8..];
        if let Some(after_dot) = cursor.strip_prefix('.') {
            let digit_count = after_dot.chars().take_while(char::is_ascii_digit).count();
            if digit_count == 0 || digit_count > 9 {
                return Err(TimeError::InvalidFractionalSeconds {
                    found: after_dot.chars().take(digit_count.max(1)).collect(),
                });
            }
            cursor = &after_dot[digit_count..];
        }

        if !cursor.is_empty() {
            return Err(TimeError::UnexpectedCharacter {
                char: cursor.chars().next().expect("non-empty checked above"),
                index: value.len() - cursor.len(),
            });
        }

        Ok(())
    }

    /// Creates a new `Time` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `time` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Time;
    ///
    /// let time = Time::new_unchecked("14:30:00");
    /// assert_eq!(time.as_str(), "14:30:00");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for Time {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "time".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Time {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Time {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Time> for String {
    fn from(value: Time) -> Self {
        value.0
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
