use std::str::FromStr;

use super::*;

#[test]
fn test_valid_times() {
    assert_eq!(Time::validate("00:00:00"), Ok(()));
    assert_eq!(Time::validate("23:59:59"), Ok(()));
    assert_eq!(Time::validate("14:30:00.123"), Ok(()));
    assert_eq!(Time::validate("14:30:00.123456789"), Ok(()));
    assert_eq!(Time::validate("14:30:00.1"), Ok(()));
}

#[test]
fn test_leap_second() {
    assert_eq!(Time::validate("23:59:60"), Ok(()));
}

#[test]
fn test_empty() {
    assert_eq!(Time::validate(""), Err(TimeError::Empty));
}

#[test]
fn test_incomplete() {
    assert_eq!(
        Time::validate("12:00"),
        Err(TimeError::Incomplete {
            found: "12:00".to_owned()
        })
    );
    assert_eq!(
        Time::validate("12"),
        Err(TimeError::Incomplete {
            found: "12".to_owned()
        })
    );
}

#[test]
fn test_midnight_24_forbidden() {
    assert_eq!(
        Time::validate("24:00:00"),
        Err(TimeError::HourOutOfRange { hour: 24 })
    );
}

#[test]
fn test_hour_out_of_range() {
    assert_eq!(
        Time::validate("25:00:00"),
        Err(TimeError::HourOutOfRange { hour: 25 })
    );
}

#[test]
fn test_minute_out_of_range() {
    assert_eq!(
        Time::validate("12:60:00"),
        Err(TimeError::MinuteOutOfRange { minute: 60 })
    );
}

#[test]
fn test_second_out_of_range() {
    assert_eq!(
        Time::validate("12:00:61"),
        Err(TimeError::SecondOutOfRange { second: 61 })
    );
}

#[test]
fn test_negative_time_rejected() {
    assert_eq!(
        Time::validate("-01:00:00"),
        Err(TimeError::InvalidHour {
            found: "-0".to_owned()
        })
    );
}

#[test]
fn test_invalid_fractional_seconds() {
    assert_eq!(
        Time::validate("12:00:00.1234567890"),
        Err(TimeError::InvalidFractionalSeconds {
            found: "1234567890".to_owned()
        })
    );
    assert_eq!(
        Time::validate("12:00:00."),
        Err(TimeError::InvalidFractionalSeconds {
            found: String::new()
        })
    );
}

#[test]
fn test_timezone_forbidden() {
    assert_eq!(
        Time::validate("14:30:00Z"),
        Err(TimeError::UnexpectedCharacter {
            char: 'Z',
            index: 8
        })
    );
    assert_eq!(
        Time::validate("14:30:00+02:00"),
        Err(TimeError::UnexpectedCharacter {
            char: '+',
            index: 8
        })
    );
    assert_eq!(
        Time::validate("14:30:00.5Z"),
        Err(TimeError::UnexpectedCharacter {
            char: 'Z',
            index: 10
        })
    );
}

#[test]
fn test_error_message_output() {
    let result = Time::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'time': time must not be empty"
    );

    let midnight = Time::try_from("24:00:00");
    assert!(midnight.is_err());
    assert_eq!(
        midnight.unwrap_err().to_string(),
        "invalid value '24:00:00' for type 'time': hour 24 out of range: must be 00-23 ('24:00' is forbidden)"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    let from_str = Time::new("14:30:00");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "14:30:00");

    let from_string = Time::new(String::from("14:30:00"));
    assert!(from_string.is_ok());

    let boxed_str: Box<str> = "14:30:00".into();
    let from_boxed = Time::new(boxed_str);
    assert!(from_boxed.is_ok());

    let invalid = Time::new("24:00:00");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let time = Time::new_unchecked("14:30:00");
    assert_eq!(time.as_str(), "14:30:00");
    assert_eq!(time.into_inner(), "14:30:00".to_string());

    let unvalidated = Time::new_unchecked("not-a-time");
    assert_eq!(unvalidated.as_str(), "not-a-time");
}

#[test]
fn test_from_str_and_display() {
    let time = Time::from_str("14:30:00").unwrap();
    assert_eq!(time.as_str(), "14:30:00");
    assert_eq!(format!("{time}"), "14:30:00");

    assert!(Time::from_str("").is_err());
}

#[test]
fn test_from_time_into_string() {
    let time = Time::new("14:30:00").unwrap();
    let s: String = time.into();
    assert_eq!(s, "14:30:00");
}

#[test]
fn test_ordering_and_equality() {
    let a = Time::new_unchecked("00:00:00");
    let b = Time::new_unchecked("23:59:59");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    let time = Time::new("14:30:00").unwrap();
    let json = serde_json::to_string(&time).unwrap();
    assert_eq!(json, "\"14:30:00\"");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"14:30:00\"";
    let time: Time = serde_json::from_str(json).unwrap();
    assert_eq!(time.as_str(), "14:30:00");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"14:30:00\"";
    let time: Time = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(time.as_str(), "14:30:00");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("14:30:00".to_string());
    let time: Time = serde_json::from_value(val).unwrap();
    assert_eq!(time.as_str(), "14:30:00");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Time>("\"\"").is_err());
    assert!(serde_json::from_str::<Time>("\"24:00:00\"").is_err());
    assert!(serde_json::from_str::<Time>("\"14:30:00Z\"").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Time>("123").is_err());
    assert!(serde_json::from_str::<Time>("true").is_err());
    assert!(serde_json::from_str::<Time>("null").is_err());
    assert!(serde_json::from_str::<Time>("[]").is_err());
    assert!(serde_json::from_str::<Time>("{}").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let original = Time::new("14:30:00.5").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Time = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_time_error_display_formatting() {
    let err = TimeError::SecondOutOfRange { second: 61 };
    assert_eq!(
        err.to_string(),
        "second 61 out of range: must be 00-60 (60 permitted for leap seconds)"
    );
}
