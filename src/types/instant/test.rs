use std::str::FromStr;

use super::*;

#[test]
fn test_valid_instants() {
    assert_eq!(Instant::validate("2017-01-01T00:00:00Z"), Ok(()));
    assert_eq!(Instant::validate("2015-02-07T13:28:17.239+02:00"), Ok(()));
    assert_eq!(Instant::validate("2020-05-15T23:59:59Z"), Ok(()));
    assert_eq!(Instant::validate("2020-05-15T10:30:00.1Z"), Ok(()));
    assert_eq!(Instant::validate("2020-05-15T10:30:00.123456789Z"), Ok(()));
}

#[test]
fn test_leap_second() {
    assert_eq!(Instant::validate("2020-05-15T23:59:60Z"), Ok(()));
}

#[test]
fn test_timezone_boundaries() {
    assert_eq!(Instant::validate("2020-05-15T10:30:00+14:00"), Ok(()));
    assert_eq!(Instant::validate("2020-05-15T10:30:00-14:00"), Ok(()));
    assert_eq!(
        Instant::validate("2020-05-15T10:30:00+14:01"),
        Err(InstantError::InvalidTimezone {
            found: "+14:01".to_owned()
        })
    );
}

#[test]
fn test_invalid_year() {
    assert_eq!(
        Instant::validate("202"),
        Err(InstantError::InvalidYear {
            found: "202".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("20X0"),
        Err(InstantError::InvalidYear {
            found: "20X0".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("0000"),
        Err(InstantError::InvalidYear {
            found: "0000".to_owned()
        })
    );
}

#[test]
fn test_invalid_month() {
    assert_eq!(
        Instant::validate("2020-1"),
        Err(InstantError::InvalidMonth {
            found: "1".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("2020-1X"),
        Err(InstantError::InvalidMonth {
            found: "1X".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("2020-13"),
        Err(InstantError::MonthOutOfRange { month: 13 })
    );
    assert_eq!(
        Instant::validate("2020-05X15"),
        Err(InstantError::UnexpectedCharacter {
            char: 'X',
            index: 7
        })
    );
}

#[test]
fn test_invalid_day() {
    assert_eq!(
        Instant::validate("2020-05-1"),
        Err(InstantError::InvalidDay {
            found: "1".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("2020-05-1X"),
        Err(InstantError::InvalidDay {
            found: "1X".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("2020-05-32"),
        Err(InstantError::DayOutOfRange { day: 32 })
    );
}

#[test]
fn test_thirty_day_month_calendar_validity() {
    assert_eq!(Instant::validate("2020-04-30T00:00:00Z"), Ok(()));
    assert_eq!(
        Instant::validate("2020-04-31T00:00:00Z"),
        Err(InstantError::InvalidCalendarDate {
            year: 2020,
            month: 4,
            day: 31
        })
    );
}

#[test]
fn test_invalid_hour_and_missing_colon_separators() {
    assert_eq!(
        Instant::validate("2020-05-15TXX:30:00Z"),
        Err(InstantError::InvalidHour {
            found: "XX".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("2020-05-15T10X30:00Z"),
        Err(InstantError::UnexpectedCharacter {
            char: 'X',
            index: 13
        })
    );
    assert_eq!(
        Instant::validate("2020-05-15T10:XX:00Z"),
        Err(InstantError::InvalidMinute {
            found: "XX".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("2020-05-15T10:30X00Z"),
        Err(InstantError::UnexpectedCharacter {
            char: 'X',
            index: 16
        })
    );
    assert_eq!(
        Instant::validate("2020-05-15T10:30:XXZ"),
        Err(InstantError::InvalidSecond {
            found: "XX".to_owned()
        })
    );
}

#[test]
fn test_minute_out_of_range() {
    assert_eq!(
        Instant::validate("2020-05-15T10:60:00Z"),
        Err(InstantError::MinuteOutOfRange { minute: 60 })
    );
}

#[test]
fn test_second_out_of_range() {
    assert_eq!(
        Instant::validate("2020-05-15T10:30:61Z"),
        Err(InstantError::SecondOutOfRange { second: 61 })
    );
}

#[test]
fn test_invalid_timezone_shape() {
    assert_eq!(
        Instant::validate("2020-05-15T10:30:00+2:00"),
        Err(InstantError::InvalidTimezone {
            found: "+2:00".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("2020-05-15T10:30:00+02-00"),
        Err(InstantError::InvalidTimezone {
            found: "+02-00".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("2020-05-15T10:30:00+XX:00"),
        Err(InstantError::InvalidTimezone {
            found: "+XX:00".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("2020-05-15T10:30:00+02:XX"),
        Err(InstantError::InvalidTimezone {
            found: "+02:XX".to_owned()
        })
    );
}

#[test]
fn test_leap_year() {
    assert_eq!(Instant::validate("2020-02-29T00:00:00Z"), Ok(()));
    assert_eq!(
        Instant::validate("2019-02-29T00:00:00Z"),
        Err(InstantError::InvalidCalendarDate {
            year: 2019,
            month: 2,
            day: 29
        })
    );
}

#[test]
fn test_empty() {
    assert_eq!(Instant::validate(""), Err(InstantError::Empty));
}

#[test]
fn test_partial_date_rejected() {
    assert_eq!(
        Instant::validate("2020"),
        Err(InstantError::Incomplete {
            found: String::new()
        })
    );
    assert_eq!(
        Instant::validate("2020-05"),
        Err(InstantError::Incomplete {
            found: String::new()
        })
    );
    assert_eq!(
        Instant::validate("2020-05-15"),
        Err(InstantError::Incomplete {
            found: String::new()
        })
    );
}

#[test]
fn test_missing_seconds() {
    assert_eq!(
        Instant::validate("2020-05-15T10:30Z"),
        Err(InstantError::IncompleteTime {
            found: "10:30Z".to_owned()
        })
    );
}

#[test]
fn test_missing_timezone() {
    assert_eq!(
        Instant::validate("2020-05-15T10:30:00"),
        Err(InstantError::MissingTimezone)
    );
}

#[test]
fn test_invalid_calendar_date() {
    assert_eq!(
        Instant::validate("2020-02-30T00:00:00Z"),
        Err(InstantError::InvalidCalendarDate {
            year: 2020,
            month: 2,
            day: 30
        })
    );
}

#[test]
fn test_hour_out_of_range() {
    assert_eq!(
        Instant::validate("2020-05-15T24:00:00Z"),
        Err(InstantError::HourOutOfRange { hour: 24 })
    );
}

#[test]
fn test_invalid_fractional_seconds() {
    assert_eq!(
        Instant::validate("2020-05-15T10:30:00.1234567890Z"),
        Err(InstantError::InvalidFractionalSeconds {
            found: "1234567890".to_owned()
        })
    );
}

#[test]
fn test_invalid_timezone() {
    assert_eq!(
        Instant::validate("2020-05-15T10:30:00+25:00"),
        Err(InstantError::InvalidTimezone {
            found: "+25:00".to_owned()
        })
    );
    assert_eq!(
        Instant::validate("2020-05-15T10:30:00X"),
        Err(InstantError::InvalidTimezone {
            found: "X".to_owned()
        })
    );
}

#[test]
fn test_unexpected_character() {
    assert_eq!(
        Instant::validate("2020/05-15T10:30:00Z"),
        Err(InstantError::UnexpectedCharacter {
            char: '/',
            index: 4
        })
    );
    assert_eq!(
        Instant::validate("2020-05-15X10:30:00Z"),
        Err(InstantError::UnexpectedCharacter {
            char: 'X',
            index: 10
        })
    );
}

#[test]
fn test_error_message_output() {
    let result = Instant::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'instant': instant must not be empty"
    );

    let no_tz = Instant::try_from("2020-05-15T10:30:00");
    assert!(no_tz.is_err());
    assert_eq!(
        no_tz.unwrap_err().to_string(),
        "invalid value '2020-05-15T10:30:00' for type 'instant': timezone offset is required"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    let from_str = Instant::new("2020-05-15T10:30:00Z");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "2020-05-15T10:30:00Z");

    let from_string = Instant::new(String::from("2020-05-15T10:30:00Z"));
    assert!(from_string.is_ok());

    let boxed_str: Box<str> = "2020-05-15T10:30:00Z".into();
    let from_boxed = Instant::new(boxed_str);
    assert!(from_boxed.is_ok());

    let invalid = Instant::new("2020-05-15");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let instant = Instant::new_unchecked("2024-01-01T00:00:00Z");
    assert_eq!(instant.as_str(), "2024-01-01T00:00:00Z");
    assert_eq!(instant.into_inner(), "2024-01-01T00:00:00Z".to_string());

    let unvalidated = Instant::new_unchecked("not-an-instant");
    assert_eq!(unvalidated.as_str(), "not-an-instant");
}

#[test]
fn test_from_str_and_display() {
    let instant = Instant::from_str("2020-05-15T10:30:00Z").unwrap();
    assert_eq!(instant.as_str(), "2020-05-15T10:30:00Z");
    assert_eq!(format!("{instant}"), "2020-05-15T10:30:00Z");

    assert!(Instant::from_str("").is_err());
}

#[test]
fn test_from_instant_into_string() {
    let instant = Instant::new("2020-05-15T10:30:00Z").unwrap();
    let s: String = instant.into();
    assert_eq!(s, "2020-05-15T10:30:00Z");
}

#[test]
fn test_ordering_and_equality() {
    let a = Instant::new_unchecked("2020-01-01T00:00:00Z");
    let b = Instant::new_unchecked("2021-01-01T00:00:00Z");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    let instant = Instant::new("2020-05-15T10:30:00Z").unwrap();
    let json = serde_json::to_string(&instant).unwrap();
    assert_eq!(json, "\"2020-05-15T10:30:00Z\"");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"2020-05-15T10:30:00Z\"";
    let instant: Instant = serde_json::from_str(json).unwrap();
    assert_eq!(instant.as_str(), "2020-05-15T10:30:00Z");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"2020-05-15T10:30:00Z\"";
    let instant: Instant = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(instant.as_str(), "2020-05-15T10:30:00Z");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("2020-05-15T10:30:00Z".to_string());
    let instant: Instant = serde_json::from_value(val).unwrap();
    assert_eq!(instant.as_str(), "2020-05-15T10:30:00Z");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Instant>("\"\"").is_err());
    assert!(serde_json::from_str::<Instant>("\"2020-05-15\"").is_err());
    assert!(serde_json::from_str::<Instant>("\"2020-05-15T10:30:00\"").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Instant>("123").is_err());
    assert!(serde_json::from_str::<Instant>("true").is_err());
    assert!(serde_json::from_str::<Instant>("null").is_err());
    assert!(serde_json::from_str::<Instant>("[]").is_err());
    assert!(serde_json::from_str::<Instant>("{}").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let original = Instant::new("2015-02-07T13:28:17.239+02:00").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Instant = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_instant_error_display_formatting() {
    assert_eq!(InstantError::Empty.to_string(), "instant must not be empty");
    assert_eq!(
        InstantError::Incomplete {
            found: String::new()
        }
        .to_string(),
        "incomplete instant '': full date and time are required"
    );
    assert_eq!(
        InstantError::InvalidYear {
            found: "202".to_owned()
        }
        .to_string(),
        "invalid year segment '202': must be 4 digits, not '0000'"
    );
    assert_eq!(
        InstantError::InvalidMonth {
            found: "1X".to_owned()
        }
        .to_string(),
        "invalid month segment '1X': must be 2 digits"
    );
    assert_eq!(
        InstantError::MonthOutOfRange { month: 13 }.to_string(),
        "month 13 out of range: must be 01-12"
    );
    assert_eq!(
        InstantError::InvalidDay {
            found: "1X".to_owned()
        }
        .to_string(),
        "invalid day segment '1X': must be 2 digits"
    );
    assert_eq!(
        InstantError::DayOutOfRange { day: 32 }.to_string(),
        "day 32 out of range: must be 01-31"
    );
    assert_eq!(
        InstantError::InvalidCalendarDate {
            year: 2020,
            month: 2,
            day: 30
        }
        .to_string(),
        "2020-02-30 is not a valid calendar date"
    );
    assert_eq!(
        InstantError::IncompleteTime {
            found: "10:30".to_owned()
        }
        .to_string(),
        "incomplete time-of-day '10:30': expected hh:mm:ss"
    );
    assert_eq!(
        InstantError::InvalidHour {
            found: "XX".to_owned()
        }
        .to_string(),
        "invalid hour segment 'XX': must be 2 digits"
    );
    assert_eq!(
        InstantError::HourOutOfRange { hour: 24 }.to_string(),
        "hour 24 out of range: must be 00-23"
    );
    assert_eq!(
        InstantError::InvalidMinute {
            found: "XX".to_owned()
        }
        .to_string(),
        "invalid minute segment 'XX': must be 2 digits"
    );
    assert_eq!(
        InstantError::MinuteOutOfRange { minute: 60 }.to_string(),
        "minute 60 out of range: must be 00-59"
    );
    assert_eq!(
        InstantError::InvalidSecond {
            found: "XX".to_owned()
        }
        .to_string(),
        "invalid second segment 'XX': must be 2 digits"
    );
    assert_eq!(
        InstantError::SecondOutOfRange { second: 61 }.to_string(),
        "second 61 out of range: must be 00-60 (60 permitted for leap seconds)"
    );
    assert_eq!(
        InstantError::InvalidFractionalSeconds {
            found: String::new()
        }
        .to_string(),
        "invalid fractional seconds '': must be 1-9 digits"
    );
    assert_eq!(
        InstantError::MissingTimezone.to_string(),
        "timezone offset is required"
    );
    assert_eq!(
        InstantError::InvalidTimezone {
            found: "X".to_owned()
        }
        .to_string(),
        "invalid timezone offset 'X': must be 'Z' or '(+|-)hh:mm' in -14:00..=+14:00"
    );
    assert_eq!(
        InstantError::UnexpectedCharacter {
            char: '/',
            index: 4
        }
        .to_string(),
        "unexpected character '/' at byte index 4"
    );
}
