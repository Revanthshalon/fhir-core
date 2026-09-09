//! A module for the FHIR `date` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#date)
//! - Regex: `([0-9]([0-9]([0-9][1-9]|[1-9]0)|[1-9]00)|[1-9]000)(-(0[1-9]|1[0-2])(-(0[1-9]|[1-2][0-9]|3[0-1]))?)?`
//! - "A date, or partial date (e.g. just year or year + month) as used in human
//!   communication. The format is YYYY, YYYY-MM, or YYYY-MM-DD, e.g. 2018, 1973-06, or
//!   1905-08-23."
//! - Year `"0000"` is excluded by the regex (the year digits may not all be zero).
//! - "There SHALL be no timezone offset" and "Dates SHALL be valid dates" — the latter
//!   is a normative rule separate from the regex (which permits `31` as a day for every
//!   month and does not know about leap years), so calendar validity (day count per
//!   month, including Feb 29 in leap years) is checked as an explicit second pass here
//!   rather than folded into a single scan.
//! - JSON encoding: a JSON string.
//!
//! # Usage
//! To create a new [`Date`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`Date::new_unchecked`] when the input is already known to be valid.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `date` primitive data type.
///
/// A full or partial calendar date: `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`. No timezone
/// offset is permitted.
///
/// # Invariants
/// Any instance of `Date` is guaranteed to satisfy:
/// - Year is exactly 4 ASCII digits and not `"0000"`.
/// - If present, month is `01`-`12`.
/// - If present, day is a valid day for the given year and month (including leap years).
///
/// # Examples
/// ```
/// use fhir_core::types::Date;
///
/// let valid = Date::new("1905-08-23");
/// assert!(valid.is_ok());
///
/// let invalid = Date::new("2020-02-30");
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
pub struct Date(String);

#[cfg(feature = "serde")]
impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `date`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateError {
    /// The value is empty.
    Empty,
    /// The year segment is not exactly 4 ASCII digits, or is `"0000"`.
    InvalidYear {
        /// The offending year segment (or the whole value, if shorter than 4 characters).
        found: String,
    },
    /// The month segment is not exactly 2 ASCII digits.
    InvalidMonth {
        /// The offending month segment.
        found: String,
    },
    /// The month segment was 2 digits but not in `01..=12`.
    MonthOutOfRange {
        /// The out-of-range month value.
        month: u32,
    },
    /// The day segment is not exactly 2 ASCII digits.
    InvalidDay {
        /// The offending day segment.
        found: String,
    },
    /// The day segment was 2 digits but not in `01..=31`.
    DayOutOfRange {
        /// The out-of-range day value.
        day: u32,
    },
    /// The day does not exist in the given month/year (e.g. `2020-02-30`, or `2019-02-29`
    /// outside a leap year).
    InvalidCalendarDate {
        /// The year component.
        year: u32,
        /// The month component.
        month: u32,
        /// The day component that doesn't exist for this year/month.
        day: u32,
    },
    /// An unexpected character was found where a `-` separator or end-of-string was expected.
    UnexpectedCharacter {
        /// The unexpected character.
        char: char,
        /// The zero-based byte index of the character.
        index: usize,
    },
}

impl std::fmt::Display for DateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateError::Empty => write!(f, "date must not be empty"),
            DateError::InvalidYear { found } => {
                write!(
                    f,
                    "invalid year segment '{found}': must be 4 digits, not '0000'"
                )
            }
            DateError::InvalidMonth { found } => {
                write!(f, "invalid month segment '{found}': must be 2 digits")
            }
            DateError::MonthOutOfRange { month } => {
                write!(f, "month {month} out of range: must be 01-12")
            }
            DateError::InvalidDay { found } => {
                write!(f, "invalid day segment '{found}': must be 2 digits")
            }
            DateError::DayOutOfRange { day } => {
                write!(f, "day {day} out of range: must be 01-31")
            }
            DateError::InvalidCalendarDate { year, month, day } => {
                write!(
                    f,
                    "{year:04}-{month:02}-{day:02} is not a valid calendar date"
                )
            }
            DateError::UnexpectedCharacter { char, index } => {
                write!(f, "unexpected character '{char}' at byte index {index}")
            }
        }
    }
}

impl std::error::Error for DateError {}

fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => unreachable!("month is validated to be in 1..=12 before this is called"),
    }
}

impl Date {
    /// Creates a new `Date` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input does not conform to the FHIR `date` specification.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Date;
    ///
    /// let date = Date::new("1905-08-23").unwrap();
    /// assert_eq!(date.as_str(), "1905-08-23");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying `date` value.
    ///
    /// `Date` intentionally exposes `as_str` rather than implementing `Deref<Target = str>`
    /// or `AsRef<str>` to prevent unwanted conversions that bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `Date` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `date` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string is `YYYY`, `YYYY-MM`, or `YYYY-MM-DD` and, when a full
    ///   date is given, the day is valid for that year/month.
    /// - `Err(DateError)` describing the exact failure reason if invalid.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Date;
    ///
    /// assert!(Date::validate("2020-02-29").is_ok()); // leap year
    /// assert!(Date::validate("2020-02-30").is_err());
    /// ```
    pub fn validate(value: &str) -> Result<(), DateError> {
        if value.is_empty() {
            return Err(DateError::Empty);
        }

        if value.len() < 4 || !value.as_bytes()[..4].iter().all(u8::is_ascii_digit) {
            return Err(DateError::InvalidYear {
                found: value.to_owned(),
            });
        }
        let year_str = &value[0..4];
        if year_str == "0000" {
            return Err(DateError::InvalidYear {
                found: year_str.to_owned(),
            });
        }
        let year: u32 = year_str.parse().expect("checked all-ASCII-digit above");

        let after_year = &value[4..];
        if after_year.is_empty() {
            return Ok(());
        }
        if !after_year.starts_with('-') {
            return Err(DateError::UnexpectedCharacter {
                char: after_year.chars().next().expect("non-empty checked above"),
                index: 4,
            });
        }

        let after_dash1 = &after_year[1..];
        if after_dash1.len() < 2 || !after_dash1.as_bytes()[..2].iter().all(u8::is_ascii_digit) {
            return Err(DateError::InvalidMonth {
                found: after_dash1.to_owned(),
            });
        }
        let month_str = &after_dash1[0..2];
        let month: u32 = month_str.parse().expect("checked all-ASCII-digit above");
        if !(1..=12).contains(&month) {
            return Err(DateError::MonthOutOfRange { month });
        }

        let after_month = &after_dash1[2..];
        if after_month.is_empty() {
            return Ok(());
        }
        if !after_month.starts_with('-') {
            return Err(DateError::UnexpectedCharacter {
                char: after_month.chars().next().expect("non-empty checked above"),
                index: 7,
            });
        }

        let day_str = &after_month[1..];
        if day_str.len() != 2 || !day_str.as_bytes().iter().all(u8::is_ascii_digit) {
            return Err(DateError::InvalidDay {
                found: day_str.to_owned(),
            });
        }
        let day: u32 = day_str.parse().expect("checked all-ASCII-digit above");
        if !(1..=31).contains(&day) {
            return Err(DateError::DayOutOfRange { day });
        }

        if day > days_in_month(year, month) {
            return Err(DateError::InvalidCalendarDate { year, month, day });
        }

        Ok(())
    }

    /// Creates a new `Date` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `date` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::Date;
    ///
    /// let date = Date::new_unchecked("2024-01-01");
    /// assert_eq!(date.as_str(), "2024-01-01");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for Date {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "date".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Date {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for Date {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<Date> for String {
    fn from(value: Date) -> Self {
        value.0
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
