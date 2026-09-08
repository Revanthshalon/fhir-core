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

#[test]
fn test_serde_serialization() {
    let instant = Instant::new("2020-05-15T10:30:00Z").unwrap();
    let json = serde_json::to_string(&instant).unwrap();
    assert_eq!(json, "\"2020-05-15T10:30:00Z\"");
}

#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"2020-05-15T10:30:00Z\"";
    let instant: Instant = serde_json::from_str(json).unwrap();
    assert_eq!(instant.as_str(), "2020-05-15T10:30:00Z");
}

#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"2020-05-15T10:30:00Z\"";
    let instant: Instant = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(instant.as_str(), "2020-05-15T10:30:00Z");
}

#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("2020-05-15T10:30:00Z".to_string());
    let instant: Instant = serde_json::from_value(val).unwrap();
    assert_eq!(instant.as_str(), "2020-05-15T10:30:00Z");
}

#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Instant>("\"\"").is_err());
    assert!(serde_json::from_str::<Instant>("\"2020-05-15\"").is_err());
    assert!(serde_json::from_str::<Instant>("\"2020-05-15T10:30:00\"").is_err());
}

#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Instant>("123").is_err());
    assert!(serde_json::from_str::<Instant>("true").is_err());
    assert!(serde_json::from_str::<Instant>("null").is_err());
    assert!(serde_json::from_str::<Instant>("[]").is_err());
    assert!(serde_json::from_str::<Instant>("{}").is_err());
}

#[test]
fn test_serde_roundtrip() {
    let original = Instant::new("2015-02-07T13:28:17.239+02:00").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Instant = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_instant_error_display_formatting() {
    let err = InstantError::MissingTimezone;
    assert_eq!(err.to_string(), "timezone offset is required");
}
