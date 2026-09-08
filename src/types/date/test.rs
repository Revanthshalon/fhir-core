use std::str::FromStr;

use super::*;

#[test]
fn test_valid_dates() {
    assert_eq!(Date::validate("2020"), Ok(()));
    assert_eq!(Date::validate("2020-05"), Ok(()));
    assert_eq!(Date::validate("2020-05-15"), Ok(()));
    assert_eq!(Date::validate("1905-08-23"), Ok(()));
    assert_eq!(Date::validate("0001"), Ok(()));
    assert_eq!(Date::validate("2020-01"), Ok(()));
    assert_eq!(Date::validate("2020-12"), Ok(()));
    assert_eq!(Date::validate("2020-01-01"), Ok(()));
    assert_eq!(Date::validate("2020-12-31"), Ok(()));
}

#[test]
fn test_leap_year() {
    assert_eq!(Date::validate("2020-02-29"), Ok(())); // divisible by 4, not 100
    assert_eq!(Date::validate("2000-02-29"), Ok(())); // divisible by 400
    assert_eq!(
        Date::validate("2019-02-29"),
        Err(DateError::InvalidCalendarDate {
            year: 2019,
            month: 2,
            day: 29
        })
    );
    assert_eq!(
        Date::validate("1900-02-29"),
        Err(DateError::InvalidCalendarDate {
            year: 1900,
            month: 2,
            day: 29
        })
    ); // divisible by 100, not 400
}

#[test]
fn test_empty() {
    assert_eq!(Date::validate(""), Err(DateError::Empty));
}

#[test]
fn test_invalid_year() {
    assert_eq!(
        Date::validate("0000"),
        Err(DateError::InvalidYear {
            found: "0000".to_owned()
        })
    );
    assert_eq!(
        Date::validate("202"),
        Err(DateError::InvalidYear {
            found: "202".to_owned()
        })
    );
    assert_eq!(
        Date::validate("20X0"),
        Err(DateError::InvalidYear {
            found: "20X0".to_owned()
        })
    );
}

#[test]
fn test_invalid_month() {
    assert_eq!(
        Date::validate("2020-5"),
        Err(DateError::InvalidMonth {
            found: "5".to_owned()
        })
    );
    assert_eq!(
        Date::validate("2020-1X"),
        Err(DateError::InvalidMonth {
            found: "1X".to_owned()
        })
    );
    assert_eq!(
        Date::validate("2020-00"),
        Err(DateError::MonthOutOfRange { month: 0 })
    );
    assert_eq!(
        Date::validate("2020-13"),
        Err(DateError::MonthOutOfRange { month: 13 })
    );
}

#[test]
fn test_invalid_day() {
    assert_eq!(
        Date::validate("2020-05-1"),
        Err(DateError::InvalidDay {
            found: "1".to_owned()
        })
    );
    assert_eq!(
        Date::validate("2020-05-1X"),
        Err(DateError::InvalidDay {
            found: "1X".to_owned()
        })
    );
    assert_eq!(
        Date::validate("2020-05-00"),
        Err(DateError::DayOutOfRange { day: 0 })
    );
    assert_eq!(
        Date::validate("2020-05-32"),
        Err(DateError::DayOutOfRange { day: 32 })
    );
}

#[test]
fn test_invalid_calendar_date() {
    assert_eq!(
        Date::validate("2020-02-30"),
        Err(DateError::InvalidCalendarDate {
            year: 2020,
            month: 2,
            day: 30
        })
    );
    assert_eq!(
        Date::validate("2020-04-31"),
        Err(DateError::InvalidCalendarDate {
            year: 2020,
            month: 4,
            day: 31
        })
    );
}

#[test]
fn test_unexpected_character() {
    assert_eq!(
        Date::validate("2020/05"),
        Err(DateError::UnexpectedCharacter {
            char: '/',
            index: 4
        })
    );
    assert_eq!(
        Date::validate("2020-05/15"),
        Err(DateError::UnexpectedCharacter {
            char: '/',
            index: 7
        })
    );
}

#[test]
fn test_error_message_output() {
    let result = Date::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'date': date must not be empty"
    );

    let bad = Date::try_from("2020-02-30");
    assert!(bad.is_err());
    assert_eq!(
        bad.unwrap_err().to_string(),
        "invalid value '2020-02-30' for type 'date': 2020-02-30 is not a valid calendar date"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    let from_str = Date::new("2020-05-15");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "2020-05-15");

    let from_string = Date::new(String::from("2020"));
    assert!(from_string.is_ok());
    assert_eq!(from_string.unwrap().as_str(), "2020");

    let boxed_str: Box<str> = "2020-05".into();
    let from_boxed = Date::new(boxed_str);
    assert!(from_boxed.is_ok());
    assert_eq!(from_boxed.unwrap().as_str(), "2020-05");

    let invalid = Date::new("0000");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let date = Date::new_unchecked("2024-01-01");
    assert_eq!(date.as_str(), "2024-01-01");
    assert_eq!(date.into_inner(), "2024-01-01".to_string());

    let unvalidated = Date::new_unchecked("not-a-date");
    assert_eq!(unvalidated.as_str(), "not-a-date");
}

#[test]
fn test_from_str_and_display() {
    let date = Date::from_str("2020-05-15").unwrap();
    assert_eq!(date.as_str(), "2020-05-15");
    assert_eq!(format!("{date}"), "2020-05-15");

    assert!(Date::from_str("").is_err());
}

#[test]
fn test_from_date_into_string() {
    let date = Date::new("2020-05-15").unwrap();
    let s: String = date.into();
    assert_eq!(s, "2020-05-15");
}

#[test]
fn test_ordering_and_equality() {
    let a = Date::new_unchecked("2020");
    let b = Date::new_unchecked("2021");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[test]
fn test_serde_serialization() {
    let date = Date::new("2020-05-15").unwrap();
    let json = serde_json::to_string(&date).unwrap();
    assert_eq!(json, "\"2020-05-15\"");
}

#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"2020-05-15\"";
    let date: Date = serde_json::from_str(json).unwrap();
    assert_eq!(date.as_str(), "2020-05-15");
}

#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"2020-05-15\"";
    let date: Date = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(date.as_str(), "2020-05-15");
}

#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("2020-05-15".to_string());
    let date: Date = serde_json::from_value(val).unwrap();
    assert_eq!(date.as_str(), "2020-05-15");
}

#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Date>("\"\"").is_err());
    assert!(serde_json::from_str::<Date>("\"0000\"").is_err());
    assert!(serde_json::from_str::<Date>("\"2020-13-01\"").is_err());
    assert!(serde_json::from_str::<Date>("\"2020-02-30\"").is_err());
}

#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Date>("123").is_err());
    assert!(serde_json::from_str::<Date>("true").is_err());
    assert!(serde_json::from_str::<Date>("null").is_err());
    assert!(serde_json::from_str::<Date>("[]").is_err());
    assert!(serde_json::from_str::<Date>("{}").is_err());
}

#[test]
fn test_serde_roundtrip() {
    let original = Date::new("1973-06-01").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Date = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_date_error_display_formatting() {
    let err = DateError::InvalidCalendarDate {
        year: 2020,
        month: 2,
        day: 30,
    };
    assert_eq!(err.to_string(), "2020-02-30 is not a valid calendar date");
}
