//! A module for the FHIR `dateTime` primitive data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#dateTime)
//! - Regex: `([0-9]([0-9]([0-9][1-9]|[1-9]0)|[1-9]00)|[1-9]000)(-(0[1-9]|1[0-2])(-(0[1-9]|[1-2][0-9]|3[0-1])(T([01][0-9]|2[0-3]):[0-5][0-9]:([0-5][0-9]|60)(\.[0-9]{1,9})?)?)?(Z|(\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?)?`
//! - "A date, date-time or partial date (e.g. just year or year + month) as used in
//!   human communication. The format is YYYY, YYYY-MM, YYYY-MM-DD or
//!   YYYY-MM-DDThh:mm:ss+zz:zz".
//! - "If hours and minutes are specified, a timezone offset SHALL be populated" — so a
//!   date-only value carries no timezone, but a value with a time-of-day always does.
//! - "Leap Seconds are allowed" — seconds may be `60`.
//! - Timezone offset range is `-14:00` to `+14:00` (only `:00` minutes at the `14:00`
//!   boundary), matching the regex's `(0[0-9]|1[0-3]):[0-5][0-9]|14:00` alternation.
//! - Fractional seconds: `\.[0-9]{1,9}`, i.e. 1 to 9 digits.
//! - "Dates SHALL be valid dates" — like [`Date`](crate::types::Date), calendar validity
//!   (leap years, day count per month) is checked as an explicit second pass, since the
//!   regex alone permits `31` for every month.
//! - JSON encoding: a JSON string.
//!
//! # Usage
//! To create a new [`DateTime`] instance:
//! - Use [`TryFrom<&str>`] or [`std::str::FromStr`] to parse and validate from a string slice.
//! - Use [`DateTime::new_unchecked`] when the input is already known to be valid.

use serde::{Deserialize, Serialize};

use crate::errors::{FhirCoreResult, r#type::TypeError};

#[cfg(test)]
mod test;

/// Represents a FHIR `dateTime` primitive data type.
///
/// A full or partial date, or a full date-time with a mandatory timezone offset:
/// `YYYY`, `YYYY-MM`, `YYYY-MM-DD`, or `YYYY-MM-DDThh:mm:ss[.sss](Z|+zz:zz|-zz:zz)`.
///
/// # Invariants
/// Any instance of `DateTime` is guaranteed to satisfy:
/// - Year is exactly 4 ASCII digits and not `"0000"`.
/// - If present, month is `01`-`12` and day is valid for the given year/month.
/// - If a time-of-day is present, it has a timezone offset; a date-only value never does.
/// - Hour `00`-`23`, minute `00`-`59`, second `00`-`60` (leap second), optional
///   fractional seconds of 1-9 digits, and timezone `Z` or `±hh:mm` in `-14:00..=+14:00`.
///
/// # Examples
/// ```
/// use fhir_core::types::DateTime;
///
/// let valid = DateTime::new("2020-05-15T10:30:00Z");
/// assert!(valid.is_ok());
///
/// let invalid = DateTime::new("2020-05-15T10:30:00"); // missing timezone
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct DateTime(String);

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Granular errors encountered while validating a FHIR `dateTime`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimeError {
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
    /// The day does not exist in the given month/year (e.g. `2020-02-30`).
    InvalidCalendarDate {
        /// The year component.
        year: u32,
        /// The month component.
        month: u32,
        /// The day component that doesn't exist for this year/month.
        day: u32,
    },
    /// The time-of-day section after `T` was shorter than the required `hh:mm:ss`.
    IncompleteTime {
        /// The offending remainder of the string after `T`.
        found: String,
    },
    /// The hour segment is not exactly 2 ASCII digits.
    InvalidHour {
        /// The offending hour segment.
        found: String,
    },
    /// The hour segment was 2 digits but not in `00..=23`.
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
    /// A time-of-day was present but no timezone offset followed it.
    MissingTimezone,
    /// The timezone offset is not `Z` or a valid `±hh:mm` in `-14:00..=+14:00`.
    InvalidTimezone {
        /// The offending timezone segment.
        found: String,
    },
    /// An unexpected character was found where a `-`, `T`, or end-of-string was expected.
    UnexpectedCharacter {
        /// The unexpected character.
        char: char,
        /// The zero-based byte index of the character.
        index: usize,
    },
}

impl std::fmt::Display for DateTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateTimeError::Empty => write!(f, "dateTime must not be empty"),
            DateTimeError::InvalidYear { found } => {
                write!(
                    f,
                    "invalid year segment '{found}': must be 4 digits, not '0000'"
                )
            }
            DateTimeError::InvalidMonth { found } => {
                write!(f, "invalid month segment '{found}': must be 2 digits")
            }
            DateTimeError::MonthOutOfRange { month } => {
                write!(f, "month {month} out of range: must be 01-12")
            }
            DateTimeError::InvalidDay { found } => {
                write!(f, "invalid day segment '{found}': must be 2 digits")
            }
            DateTimeError::DayOutOfRange { day } => {
                write!(f, "day {day} out of range: must be 01-31")
            }
            DateTimeError::InvalidCalendarDate { year, month, day } => {
                write!(
                    f,
                    "{year:04}-{month:02}-{day:02} is not a valid calendar date"
                )
            }
            DateTimeError::IncompleteTime { found } => {
                write!(f, "incomplete time-of-day '{found}': expected hh:mm:ss")
            }
            DateTimeError::InvalidHour { found } => {
                write!(f, "invalid hour segment '{found}': must be 2 digits")
            }
            DateTimeError::HourOutOfRange { hour } => {
                write!(f, "hour {hour} out of range: must be 00-23")
            }
            DateTimeError::InvalidMinute { found } => {
                write!(f, "invalid minute segment '{found}': must be 2 digits")
            }
            DateTimeError::MinuteOutOfRange { minute } => {
                write!(f, "minute {minute} out of range: must be 00-59")
            }
            DateTimeError::InvalidSecond { found } => {
                write!(f, "invalid second segment '{found}': must be 2 digits")
            }
            DateTimeError::SecondOutOfRange { second } => {
                write!(
                    f,
                    "second {second} out of range: must be 00-60 (60 permitted for leap seconds)"
                )
            }
            DateTimeError::InvalidFractionalSeconds { found } => {
                write!(
                    f,
                    "invalid fractional seconds '{found}': must be 1-9 digits"
                )
            }
            DateTimeError::MissingTimezone => {
                write!(
                    f,
                    "timezone offset is required when a time-of-day is present"
                )
            }
            DateTimeError::InvalidTimezone { found } => {
                write!(
                    f,
                    "invalid timezone offset '{found}': must be 'Z' or '(+|-)hh:mm' in -14:00..=+14:00"
                )
            }
            DateTimeError::UnexpectedCharacter { char, index } => {
                write!(f, "unexpected character '{char}' at byte index {index}")
            }
        }
    }
}

impl std::error::Error for DateTimeError {}

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

impl DateTime {
    /// Creates a new `DateTime` instance from any type that can be converted into a `String` after validation.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Type`](crate::errors::FhirCoreError::Type) containing [`TypeError::InvalidValue`]
    /// if the input does not conform to the FHIR `dateTime` specification.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::DateTime;
    ///
    /// let dt = DateTime::new("2020-05-15T10:30:00Z").unwrap();
    /// assert_eq!(dt.as_str(), "2020-05-15T10:30:00Z");
    /// ```
    pub fn new(value: impl Into<String>) -> FhirCoreResult<Self> {
        Ok(Self::try_from(value.into())?)
    }

    /// Returns a string slice of the underlying `dateTime` value.
    ///
    /// `DateTime` intentionally exposes `as_str` rather than implementing `Deref<Target = str>`
    /// or `AsRef<str>` to prevent unwanted conversions that bypass domain type semantics.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the `DateTime` wrapper and returns the underlying `String`.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Validates whether a given string is compliant with the FHIR `dateTime` format.
    ///
    /// # Returns
    /// - `Ok(())` if the string is `YYYY`, `YYYY-MM`, `YYYY-MM-DD`, or a full date-time
    ///   with a mandatory timezone offset.
    /// - `Err(DateTimeError)` describing the exact failure reason if invalid.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::DateTime;
    ///
    /// assert!(DateTime::validate("2020-05-15T10:30:00+02:00").is_ok());
    /// assert!(DateTime::validate("2020-05-15T10:30:00").is_err()); // missing timezone
    /// ```
    #[allow(clippy::too_many_lines)]
    pub fn validate(value: &str) -> Result<(), DateTimeError> {
        if value.is_empty() {
            return Err(DateTimeError::Empty);
        }

        // Year (byte offsets 0..4)
        if value.len() < 4 || !value.as_bytes()[..4].iter().all(u8::is_ascii_digit) {
            return Err(DateTimeError::InvalidYear {
                found: value.to_owned(),
            });
        }
        let year_str = &value[0..4];
        if year_str == "0000" {
            return Err(DateTimeError::InvalidYear {
                found: year_str.to_owned(),
            });
        }
        let year: u32 = year_str.parse().expect("checked all-ASCII-digit above");

        let after_year = &value[4..];
        if after_year.is_empty() {
            return Ok(());
        }
        if !after_year.starts_with('-') {
            return Err(DateTimeError::UnexpectedCharacter {
                char: after_year.chars().next().expect("non-empty checked above"),
                index: 4,
            });
        }

        // Month (byte offsets 5..7)
        let after_dash1 = &after_year[1..];
        if after_dash1.len() < 2 || !after_dash1.as_bytes()[..2].iter().all(u8::is_ascii_digit) {
            return Err(DateTimeError::InvalidMonth {
                found: after_dash1.to_owned(),
            });
        }
        let month_str = &after_dash1[0..2];
        let month: u32 = month_str.parse().expect("checked all-ASCII-digit above");
        if !(1..=12).contains(&month) {
            return Err(DateTimeError::MonthOutOfRange { month });
        }

        let after_month = &after_dash1[2..];
        if after_month.is_empty() {
            return Ok(());
        }
        if !after_month.starts_with('-') {
            return Err(DateTimeError::UnexpectedCharacter {
                char: after_month.chars().next().expect("non-empty checked above"),
                index: 7,
            });
        }

        // Day (byte offsets 8..10)
        let after_dash2 = &after_month[1..];
        if after_dash2.len() < 2 || !after_dash2.as_bytes()[..2].iter().all(u8::is_ascii_digit) {
            return Err(DateTimeError::InvalidDay {
                found: after_dash2.to_owned(),
            });
        }
        let day_str = &after_dash2[0..2];
        let day: u32 = day_str.parse().expect("checked all-ASCII-digit above");
        if !(1..=31).contains(&day) {
            return Err(DateTimeError::DayOutOfRange { day });
        }
        if day > days_in_month(year, month) {
            return Err(DateTimeError::InvalidCalendarDate { year, month, day });
        }

        let after_day = &after_dash2[2..];
        if after_day.is_empty() {
            return Ok(()); // date-only: no time-of-day, no timezone
        }
        if !after_day.starts_with('T') {
            return Err(DateTimeError::UnexpectedCharacter {
                char: after_day.chars().next().expect("non-empty checked above"),
                index: 10,
            });
        }

        // Time-of-day (byte offsets 11..19: hh:mm:ss)
        let after_t = &after_day[1..];
        if after_t.len() < 8 {
            return Err(DateTimeError::IncompleteTime {
                found: after_t.to_owned(),
            });
        }
        if !after_t.as_bytes()[0..2].iter().all(u8::is_ascii_digit) {
            return Err(DateTimeError::InvalidHour {
                found: after_t[0..2].to_owned(),
            });
        }
        let hour: u32 = after_t[0..2]
            .parse()
            .expect("checked all-ASCII-digit above");
        if hour > 23 {
            return Err(DateTimeError::HourOutOfRange { hour });
        }
        if after_t.as_bytes()[2] != b':' {
            return Err(DateTimeError::UnexpectedCharacter {
                char: after_t[2..].chars().next().expect("length checked above"),
                index: 13,
            });
        }
        if !after_t.as_bytes()[3..5].iter().all(u8::is_ascii_digit) {
            return Err(DateTimeError::InvalidMinute {
                found: after_t[3..5].to_owned(),
            });
        }
        let minute: u32 = after_t[3..5]
            .parse()
            .expect("checked all-ASCII-digit above");
        if minute > 59 {
            return Err(DateTimeError::MinuteOutOfRange { minute });
        }
        if after_t.as_bytes()[5] != b':' {
            return Err(DateTimeError::UnexpectedCharacter {
                char: after_t[5..].chars().next().expect("length checked above"),
                index: 16,
            });
        }
        if !after_t.as_bytes()[6..8].iter().all(u8::is_ascii_digit) {
            return Err(DateTimeError::InvalidSecond {
                found: after_t[6..8].to_owned(),
            });
        }
        let second: u32 = after_t[6..8]
            .parse()
            .expect("checked all-ASCII-digit above");
        if second > 60 {
            return Err(DateTimeError::SecondOutOfRange { second });
        }

        // Optional fractional seconds, then mandatory timezone.
        let mut cursor = &after_t[8..];
        if let Some(after_dot) = cursor.strip_prefix('.') {
            let digit_count = after_dot.chars().take_while(char::is_ascii_digit).count();
            if digit_count == 0 || digit_count > 9 {
                return Err(DateTimeError::InvalidFractionalSeconds {
                    found: after_dot.chars().take(digit_count.max(1)).collect(),
                });
            }
            cursor = &after_dot[digit_count..];
        }

        if cursor.is_empty() {
            return Err(DateTimeError::MissingTimezone);
        }
        if cursor == "Z" {
            return Ok(());
        }

        let sign = cursor.chars().next().expect("non-empty checked above");
        if sign != '+' && sign != '-' {
            return Err(DateTimeError::InvalidTimezone {
                found: cursor.to_owned(),
            });
        }
        let offset = &cursor[1..];
        if offset.len() != 5 || offset.as_bytes()[2] != b':' {
            return Err(DateTimeError::InvalidTimezone {
                found: cursor.to_owned(),
            });
        }
        let tz_hour_str = &offset[0..2];
        let tz_minute_str = &offset[3..5];
        if !tz_hour_str.bytes().all(|b| b.is_ascii_digit())
            || !tz_minute_str.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(DateTimeError::InvalidTimezone {
                found: cursor.to_owned(),
            });
        }
        let tz_hour: u32 = tz_hour_str.parse().expect("checked all-ASCII-digit above");
        let tz_minute: u32 = tz_minute_str
            .parse()
            .expect("checked all-ASCII-digit above");
        let valid_offset = (tz_hour <= 13 && tz_minute <= 59) || (tz_hour == 14 && tz_minute == 0);
        if !valid_offset {
            return Err(DateTimeError::InvalidTimezone {
                found: cursor.to_owned(),
            });
        }

        Ok(())
    }

    /// Creates a new `DateTime` instance without validating the input string.
    ///
    /// # Warning
    /// This bypasses validation and invariants check. The caller is responsible for ensuring
    /// that the provided value conforms to the FHIR `dateTime` format.
    ///
    /// This is typically used for performance when the input is known to be valid (e.g. compile-time
    /// constants, internal conversions, or trusted sources).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::types::DateTime;
    ///
    /// let dt = DateTime::new_unchecked("2024-01-01T00:00:00Z");
    /// assert_eq!(dt.as_str(), "2024-01-01T00:00:00Z");
    /// ```
    #[inline]
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl TryFrom<&str> for DateTime {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value).map_err(|e| TypeError::InvalidValue {
            r#type: "dateTime".to_owned(),
            value: value.to_owned(),
            error: e.to_string(),
        })?;
        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for DateTime {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl std::str::FromStr for DateTime {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<DateTime> for String {
    fn from(value: DateTime) -> Self {
        value.0
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
