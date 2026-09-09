use std::str::FromStr;

use super::*;

#[test]
fn test_valid_partial_dates() {
    assert_eq!(DateTime::validate("2020"), Ok(()));
    assert_eq!(DateTime::validate("2020-05"), Ok(()));
    assert_eq!(DateTime::validate("2020-05-15"), Ok(()));
}

#[test]
fn test_valid_full_datetimes() {
    assert_eq!(DateTime::validate("2020-05-15T10:30:00Z"), Ok(()));
    assert_eq!(DateTime::validate("2020-05-15T10:30:00+02:00"), Ok(()));
    assert_eq!(DateTime::validate("2020-05-15T10:30:00-05:00"), Ok(()));
    assert_eq!(DateTime::validate("2020-05-15T00:00:00Z"), Ok(()));
    assert_eq!(DateTime::validate("2020-05-15T23:59:59Z"), Ok(()));
    assert_eq!(DateTime::validate("2020-05-15T10:30:00.123456789Z"), Ok(()));
    assert_eq!(DateTime::validate("2020-05-15T10:30:00.1Z"), Ok(()));
}

#[test]
fn test_leap_second() {
    assert_eq!(DateTime::validate("2020-05-15T23:59:60Z"), Ok(()));
}

#[test]
fn test_timezone_boundaries() {
    assert_eq!(DateTime::validate("2020-05-15T10:30:00+14:00"), Ok(()));
    assert_eq!(DateTime::validate("2020-05-15T10:30:00-14:00"), Ok(()));
    assert_eq!(DateTime::validate("2020-05-15T10:30:00+13:59"), Ok(()));
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:00+14:01"),
        Err(DateTimeError::InvalidTimezone {
            found: "+14:01".to_owned()
        })
    );
}

#[test]
fn test_invalid_year() {
    assert_eq!(
        DateTime::validate("202"),
        Err(DateTimeError::InvalidYear {
            found: "202".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("20X0"),
        Err(DateTimeError::InvalidYear {
            found: "20X0".to_owned()
        })
    );
}

#[test]
fn test_invalid_month() {
    assert_eq!(
        DateTime::validate("2020-1"),
        Err(DateTimeError::InvalidMonth {
            found: "1".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("2020-1X"),
        Err(DateTimeError::InvalidMonth {
            found: "1X".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("2020-13"),
        Err(DateTimeError::MonthOutOfRange { month: 13 })
    );
    assert_eq!(
        DateTime::validate("2020-05X15"),
        Err(DateTimeError::UnexpectedCharacter {
            char: 'X',
            index: 7
        })
    );
}

#[test]
fn test_invalid_day() {
    assert_eq!(
        DateTime::validate("2020-05-1"),
        Err(DateTimeError::InvalidDay {
            found: "1".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-1X"),
        Err(DateTimeError::InvalidDay {
            found: "1X".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-32"),
        Err(DateTimeError::DayOutOfRange { day: 32 })
    );
}

#[test]
fn test_thirty_day_month_calendar_validity() {
    assert_eq!(DateTime::validate("2020-04-30"), Ok(()));
    assert_eq!(
        DateTime::validate("2020-04-31"),
        Err(DateTimeError::InvalidCalendarDate {
            year: 2020,
            month: 4,
            day: 31
        })
    );
}

#[test]
fn test_invalid_hour_and_missing_colon_separators() {
    assert_eq!(
        DateTime::validate("2020-05-15TXX:30:00Z"),
        Err(DateTimeError::InvalidHour {
            found: "XX".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-15T10X30:00Z"),
        Err(DateTimeError::UnexpectedCharacter {
            char: 'X',
            index: 13
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-15T10:XX:00Z"),
        Err(DateTimeError::InvalidMinute {
            found: "XX".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-15T10:30X00Z"),
        Err(DateTimeError::UnexpectedCharacter {
            char: 'X',
            index: 16
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:XXZ"),
        Err(DateTimeError::InvalidSecond {
            found: "XX".to_owned()
        })
    );
}

#[test]
fn test_invalid_timezone_shape() {
    // Wrong length / missing colon in the offset.
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:00+2:00"),
        Err(DateTimeError::InvalidTimezone {
            found: "+2:00".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:00+02-00"),
        Err(DateTimeError::InvalidTimezone {
            found: "+02-00".to_owned()
        })
    );
    // Non-digit timezone hour/minute.
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:00+XX:00"),
        Err(DateTimeError::InvalidTimezone {
            found: "+XX:00".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:00+02:XX"),
        Err(DateTimeError::InvalidTimezone {
            found: "+02:XX".to_owned()
        })
    );
}

#[test]
fn test_leap_year() {
    assert_eq!(DateTime::validate("2020-02-29"), Ok(()));
    assert_eq!(
        DateTime::validate("2019-02-29"),
        Err(DateTimeError::InvalidCalendarDate {
            year: 2019,
            month: 2,
            day: 29
        })
    );
}

#[test]
fn test_empty() {
    assert_eq!(DateTime::validate(""), Err(DateTimeError::Empty));
}

#[test]
fn test_missing_timezone() {
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:00"),
        Err(DateTimeError::MissingTimezone)
    );
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:00.123"),
        Err(DateTimeError::MissingTimezone)
    );
}

#[test]
fn test_bad_leap_year_boundary() {
    // Not a leap year: divisible by 100, not 400.
    assert_eq!(
        DateTime::validate("1900-02-29"),
        Err(DateTimeError::InvalidCalendarDate {
            year: 1900,
            month: 2,
            day: 29
        })
    );
}

#[test]
fn test_incomplete_time() {
    assert_eq!(
        DateTime::validate("2020-05-15T10:30"),
        Err(DateTimeError::IncompleteTime {
            found: "10:30".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-15T10"),
        Err(DateTimeError::IncompleteTime {
            found: "10".to_owned()
        })
    );
}

#[test]
fn test_hour_out_of_range() {
    assert_eq!(
        DateTime::validate("2020-05-15T24:00:00Z"),
        Err(DateTimeError::HourOutOfRange { hour: 24 })
    );
}

#[test]
fn test_minute_out_of_range() {
    assert_eq!(
        DateTime::validate("2020-05-15T10:60:00Z"),
        Err(DateTimeError::MinuteOutOfRange { minute: 60 })
    );
}

#[test]
fn test_second_out_of_range() {
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:61Z"),
        Err(DateTimeError::SecondOutOfRange { second: 61 })
    );
}

#[test]
fn test_invalid_fractional_seconds() {
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:00.Z"),
        Err(DateTimeError::InvalidFractionalSeconds {
            found: "Z".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:00.1234567890Z"),
        Err(DateTimeError::InvalidFractionalSeconds {
            found: "1234567890".to_owned()
        })
    );
}

#[test]
fn test_invalid_timezone() {
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:00+25:00"),
        Err(DateTimeError::InvalidTimezone {
            found: "+25:00".to_owned()
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-15T10:30:00X"),
        Err(DateTimeError::InvalidTimezone {
            found: "X".to_owned()
        })
    );
}

#[test]
fn test_unexpected_character() {
    assert_eq!(
        DateTime::validate("2020/05"),
        Err(DateTimeError::UnexpectedCharacter {
            char: '/',
            index: 4
        })
    );
    assert_eq!(
        DateTime::validate("2020-05-15X10:30:00Z"),
        Err(DateTimeError::UnexpectedCharacter {
            char: 'X',
            index: 10
        })
    );
}

#[test]
fn test_error_message_output() {
    let result = DateTime::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'dateTime': dateTime must not be empty"
    );

    let missing_tz = DateTime::try_from("2020-05-15T10:30:00");
    assert!(missing_tz.is_err());
    assert_eq!(
        missing_tz.unwrap_err().to_string(),
        "invalid value '2020-05-15T10:30:00' for type 'dateTime': timezone offset is required when a time-of-day is present"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    let from_str = DateTime::new("2020-05-15T10:30:00Z");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "2020-05-15T10:30:00Z");

    let from_string = DateTime::new(String::from("2020"));
    assert!(from_string.is_ok());
    assert_eq!(from_string.unwrap().as_str(), "2020");

    let boxed_str: Box<str> = "2020-05".into();
    let from_boxed = DateTime::new(boxed_str);
    assert!(from_boxed.is_ok());
    assert_eq!(from_boxed.unwrap().as_str(), "2020-05");

    let invalid = DateTime::new("0000");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let dt = DateTime::new_unchecked("2024-01-01T00:00:00Z");
    assert_eq!(dt.as_str(), "2024-01-01T00:00:00Z");
    assert_eq!(dt.into_inner(), "2024-01-01T00:00:00Z".to_string());

    let unvalidated = DateTime::new_unchecked("not-a-datetime");
    assert_eq!(unvalidated.as_str(), "not-a-datetime");
}

#[test]
fn test_from_str_and_display() {
    let dt = DateTime::from_str("2020-05-15T10:30:00Z").unwrap();
    assert_eq!(dt.as_str(), "2020-05-15T10:30:00Z");
    assert_eq!(format!("{dt}"), "2020-05-15T10:30:00Z");

    assert!(DateTime::from_str("").is_err());
}

#[test]
fn test_from_datetime_into_string() {
    let dt = DateTime::new("2020-05-15T10:30:00Z").unwrap();
    let s: String = dt.into();
    assert_eq!(s, "2020-05-15T10:30:00Z");
}

#[test]
fn test_ordering_and_equality() {
    let a = DateTime::new_unchecked("2020");
    let b = DateTime::new_unchecked("2021");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    let dt = DateTime::new("2020-05-15T10:30:00Z").unwrap();
    let json = serde_json::to_string(&dt).unwrap();
    assert_eq!(json, "\"2020-05-15T10:30:00Z\"");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"2020-05-15T10:30:00Z\"";
    let dt: DateTime = serde_json::from_str(json).unwrap();
    assert_eq!(dt.as_str(), "2020-05-15T10:30:00Z");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"2020-05-15T10:30:00Z\"";
    let dt: DateTime = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(dt.as_str(), "2020-05-15T10:30:00Z");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("2020-05-15T10:30:00Z".to_string());
    let dt: DateTime = serde_json::from_value(val).unwrap();
    assert_eq!(dt.as_str(), "2020-05-15T10:30:00Z");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<DateTime>("\"\"").is_err());
    assert!(serde_json::from_str::<DateTime>("\"2020-05-15T10:30:00\"").is_err());
    assert!(serde_json::from_str::<DateTime>("\"2020-13-01\"").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<DateTime>("123").is_err());
    assert!(serde_json::from_str::<DateTime>("true").is_err());
    assert!(serde_json::from_str::<DateTime>("null").is_err());
    assert!(serde_json::from_str::<DateTime>("[]").is_err());
    assert!(serde_json::from_str::<DateTime>("{}").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let original = DateTime::new("2020-05-15T10:30:00.5+02:00").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: DateTime = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_datetime_error_display_formatting() {
    assert_eq!(
        DateTimeError::Empty.to_string(),
        "dateTime must not be empty"
    );
    assert_eq!(
        DateTimeError::InvalidYear {
            found: "202".to_owned()
        }
        .to_string(),
        "invalid year segment '202': must be 4 digits, not '0000'"
    );
    assert_eq!(
        DateTimeError::InvalidMonth {
            found: "1X".to_owned()
        }
        .to_string(),
        "invalid month segment '1X': must be 2 digits"
    );
    assert_eq!(
        DateTimeError::MonthOutOfRange { month: 13 }.to_string(),
        "month 13 out of range: must be 01-12"
    );
    assert_eq!(
        DateTimeError::InvalidDay {
            found: "1X".to_owned()
        }
        .to_string(),
        "invalid day segment '1X': must be 2 digits"
    );
    assert_eq!(
        DateTimeError::DayOutOfRange { day: 32 }.to_string(),
        "day 32 out of range: must be 01-31"
    );
    assert_eq!(
        DateTimeError::InvalidCalendarDate {
            year: 2020,
            month: 2,
            day: 30
        }
        .to_string(),
        "2020-02-30 is not a valid calendar date"
    );
    assert_eq!(
        DateTimeError::IncompleteTime {
            found: "10:30".to_owned()
        }
        .to_string(),
        "incomplete time-of-day '10:30': expected hh:mm:ss"
    );
    assert_eq!(
        DateTimeError::InvalidHour {
            found: "XX".to_owned()
        }
        .to_string(),
        "invalid hour segment 'XX': must be 2 digits"
    );
    assert_eq!(
        DateTimeError::HourOutOfRange { hour: 24 }.to_string(),
        "hour 24 out of range: must be 00-23"
    );
    assert_eq!(
        DateTimeError::InvalidMinute {
            found: "XX".to_owned()
        }
        .to_string(),
        "invalid minute segment 'XX': must be 2 digits"
    );
    assert_eq!(
        DateTimeError::MinuteOutOfRange { minute: 60 }.to_string(),
        "minute 60 out of range: must be 00-59"
    );
    assert_eq!(
        DateTimeError::InvalidSecond {
            found: "XX".to_owned()
        }
        .to_string(),
        "invalid second segment 'XX': must be 2 digits"
    );
    assert_eq!(
        DateTimeError::SecondOutOfRange { second: 61 }.to_string(),
        "second 61 out of range: must be 00-60 (60 permitted for leap seconds)"
    );
    assert_eq!(
        DateTimeError::InvalidFractionalSeconds {
            found: String::new()
        }
        .to_string(),
        "invalid fractional seconds '': must be 1-9 digits"
    );
    assert_eq!(
        DateTimeError::MissingTimezone.to_string(),
        "timezone offset is required when a time-of-day is present"
    );
    assert_eq!(
        DateTimeError::InvalidTimezone {
            found: "X".to_owned()
        }
        .to_string(),
        "invalid timezone offset 'X': must be 'Z' or '(+|-)hh:mm' in -14:00..=+14:00"
    );
    assert_eq!(
        DateTimeError::UnexpectedCharacter {
            char: '/',
            index: 4
        }
        .to_string(),
        "unexpected character '/' at byte index 4"
    );
}
