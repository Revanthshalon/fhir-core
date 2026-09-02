use std::str::FromStr;

use super::*;

#[test]
fn test_valid_strings() {
    assert_eq!(FhirString::validate("Hello World"), Ok(()));
    assert_eq!(FhirString::validate("Simple text with 123!"), Ok(()));
    assert_eq!(
        FhirString::validate("Leading and trailing spaces are fine:   hello   "),
        Ok(())
    );
    assert_eq!(
        FhirString::validate("Contains\ttab,\nnewline,\rand carriage return"),
        Ok(())
    );
    assert_eq!(
        FhirString::validate("Unicode: 🚀 é ñ 漢字 مرحبا 🌍"),
        Ok(())
    );
}

#[test]
fn test_empty_and_whitespace_only() {
    assert_eq!(
        FhirString::validate(""),
        Err(StringError::EmptyOrWhitespaceOnly)
    );
    assert_eq!(
        FhirString::validate("   "),
        Err(StringError::EmptyOrWhitespaceOnly)
    );
    assert_eq!(
        FhirString::validate("\t\t"),
        Err(StringError::EmptyOrWhitespaceOnly)
    );
    assert_eq!(
        FhirString::validate("\n\r\n"),
        Err(StringError::EmptyOrWhitespaceOnly)
    );
    assert_eq!(
        FhirString::validate(" \t \n \r "),
        Err(StringError::EmptyOrWhitespaceOnly)
    );
}

#[test]
fn test_disallowed_control_characters() {
    // Null byte (0)
    assert_eq!(
        FhirString::validate("hello\0world"),
        Err(StringError::DisallowedControlCharacter {
            char: '\0',
            index: 5
        })
    );

    // SOH (1)
    assert_eq!(
        FhirString::validate("\u{0001}abc"),
        Err(StringError::DisallowedControlCharacter {
            char: '\u{0001}',
            index: 0
        })
    );

    // Backspace (8)
    assert_eq!(
        FhirString::validate("test\u{0008}"),
        Err(StringError::DisallowedControlCharacter {
            char: '\u{0008}',
            index: 4
        })
    );

    // Escape (27)
    assert_eq!(
        FhirString::validate("esc\u{001B}key"),
        Err(StringError::DisallowedControlCharacter {
            char: '\u{001B}',
            index: 3
        })
    );

    // Unit separator (31)
    assert_eq!(
        FhirString::validate("sep\u{001F}"),
        Err(StringError::DisallowedControlCharacter {
            char: '\u{001F}',
            index: 3
        })
    );
}

#[test]
fn test_max_length() {
    // Exactly at MAX_LENGTH (1,048,576 chars)
    let max_str = "a".repeat(FhirString::MAX_LENGTH);
    assert_eq!(FhirString::validate(&max_str), Ok(()));

    // Exceeding MAX_LENGTH by 1 char
    let over_max = "a".repeat(FhirString::MAX_LENGTH + 1);
    assert_eq!(
        FhirString::validate(&over_max),
        Err(StringError::ExceedsMaxLength {
            count: FhirString::MAX_LENGTH + 1,
            max: FhirString::MAX_LENGTH,
        })
    );

    // Multi-byte Unicode characters: 100,000 emojis (400,000 bytes, but 100,000 chars) is valid
    let emoji_str = "🦀".repeat(100_000);
    assert_eq!(FhirString::validate(&emoji_str), Ok(()));
}

#[test]
fn test_error_message_output() {
    let result = FhirString::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'string': string must contain non-whitespace content and cannot be empty"
    );

    let ctrl_err = FhirString::try_from("bad\0val");
    assert!(ctrl_err.is_err());
    assert_eq!(
        ctrl_err.unwrap_err().to_string(),
        "invalid value 'bad\0val' for type 'string': disallowed control character '\\0' (U+0000) at byte index 3"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    // &str
    let from_str = FhirString::new("Patient Name");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "Patient Name");

    // String
    let from_string = FhirString::new(String::from("Observation Note"));
    assert!(from_string.is_ok());
    assert_eq!(from_string.unwrap().as_str(), "Observation Note");

    // Box<str>
    let boxed_str: Box<str> = "Clinical Impression".into();
    let from_boxed = FhirString::new(boxed_str);
    assert!(from_boxed.is_ok());
    assert_eq!(from_boxed.unwrap().as_str(), "Clinical Impression");

    // Invalid input
    let invalid = FhirString::new("   ");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let s = FhirString::new_unchecked("Valid Value");
    assert_eq!(s.as_str(), "Valid Value");
    assert_eq!(s.into_inner(), "Valid Value");

    // Unchecked accepts even strings that would fail validation
    let s_unvalidated = FhirString::new_unchecked("   ");
    assert_eq!(s_unvalidated.as_str(), "   ");
}

#[test]
fn test_from_str_and_display() {
    let s = FhirString::from_str("Sample Text").unwrap();
    assert_eq!(s.as_str(), "Sample Text");
    assert_eq!(format!("{s}"), "Sample Text");

    assert!(FhirString::from_str("").is_err());
}

#[test]
fn test_serde_serialization() {
    let s = FhirString::new("Medical Record").unwrap();
    let json = serde_json::to_string(&s).unwrap();
    assert_eq!(json, "\"Medical Record\"");
}

#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"Medical Record\"";
    let s: FhirString = serde_json::from_str(json).unwrap();
    assert_eq!(s.as_str(), "Medical Record");
}

#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"Medical Record\"";
    let s: FhirString = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(s.as_str(), "Medical Record");
}

#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("Medical Record".to_string());
    let s: FhirString = serde_json::from_value(val).unwrap();
    assert_eq!(s.as_str(), "Medical Record");
}

#[test]
fn test_serde_deserialization_escaped() {
    let json = r#""\u0048\u0065\u006C\u006C\u006F""#;
    let s: FhirString = serde_json::from_str(json).unwrap();
    assert_eq!(s.as_str(), "Hello");
}

#[test]
fn test_serde_deserialization_invalid_value() {
    // Empty string
    assert!(serde_json::from_str::<FhirString>("\"\"").is_err());
    // Whitespace only
    assert!(serde_json::from_str::<FhirString>("\"   \"").is_err());
    // Disallowed control char
    assert!(serde_json::from_str::<FhirString>(r#""hello\u0000world""#).is_err());
}

#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<FhirString>("123").is_err());
    assert!(serde_json::from_str::<FhirString>("true").is_err());
    assert!(serde_json::from_str::<FhirString>("null").is_err());
    assert!(serde_json::from_str::<FhirString>("[]").is_err());
    assert!(serde_json::from_str::<FhirString>("{}").is_err());
}

#[test]
fn test_serde_roundtrip() {
    let original = FhirString::new("Roundtrip Test! 🚀").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: FhirString = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}
