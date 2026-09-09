use std::str::FromStr;

use super::*;

#[test]
fn test_valid_codes() {
    assert_eq!(Code::validate("active"), Ok(()));
    assert_eq!(Code::validate("oral-route"), Ok(()));
    assert_eq!(Code::validate("code with single spaces"), Ok(()));
    assert_eq!(Code::validate("a"), Ok(()));
    assert_eq!(Code::validate("A B C"), Ok(()));
}

#[test]
fn test_empty() {
    assert_eq!(Code::validate(""), Err(CodeError::Empty));
}

#[test]
fn test_leading_whitespace() {
    assert_eq!(
        Code::validate(" leading"),
        Err(CodeError::LeadingWhitespace)
    );
}

#[test]
fn test_trailing_whitespace() {
    assert_eq!(
        Code::validate("trailing "),
        Err(CodeError::TrailingWhitespace)
    );
}

#[test]
fn test_consecutive_whitespace() {
    assert_eq!(
        Code::validate("double  spaces"),
        Err(CodeError::ConsecutiveWhitespace { index: 7 })
    );
}

#[test]
fn test_disallowed_whitespace_character() {
    assert_eq!(
        Code::validate("tab\tseparated"),
        Err(CodeError::DisallowedWhitespaceCharacter {
            char: '\t',
            index: 3
        })
    );
    assert_eq!(
        Code::validate("new\nline"),
        Err(CodeError::DisallowedWhitespaceCharacter {
            char: '\n',
            index: 3
        })
    );
}

#[test]
fn test_single_space_only_string() {
    // A lone space: leading and (since nothing follows) also trailing — leading wins first.
    assert_eq!(Code::validate(" "), Err(CodeError::LeadingWhitespace));
}

#[test]
fn test_error_message_output() {
    let result = Code::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'code': code must not be empty"
    );

    let bad_space = Code::try_from("bad  code");
    assert!(bad_space.is_err());
    assert_eq!(
        bad_space.unwrap_err().to_string(),
        "invalid value 'bad  code' for type 'code': consecutive whitespace at byte index 4"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    let from_str = Code::new("active");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "active");

    let from_string = Code::new(String::from("oral-route"));
    assert!(from_string.is_ok());
    assert_eq!(from_string.unwrap().as_str(), "oral-route");

    let boxed_str: Box<str> = "boxed-code".into();
    let from_boxed = Code::new(boxed_str);
    assert!(from_boxed.is_ok());
    assert_eq!(from_boxed.unwrap().as_str(), "boxed-code");

    let invalid = Code::new(" bad");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let code = Code::new_unchecked("trusted-code");
    assert_eq!(code.as_str(), "trusted-code");
    assert_eq!(code.into_inner(), "trusted-code".to_string());

    let unvalidated = Code::new_unchecked(" bad code ");
    assert_eq!(unvalidated.as_str(), " bad code ");
}

#[test]
fn test_from_str_and_display() {
    let code = Code::from_str("sample-code").unwrap();
    assert_eq!(code.as_str(), "sample-code");
    assert_eq!(format!("{code}"), "sample-code");

    assert!(Code::from_str("").is_err());
}

#[test]
fn test_from_code_into_string() {
    let code = Code::new("active").unwrap();
    let s: String = code.into();
    assert_eq!(s, "active");
}

#[test]
fn test_ordering_and_equality() {
    let a = Code::new_unchecked("a");
    let b = Code::new_unchecked("b");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    let code = Code::new("active").unwrap();
    let json = serde_json::to_string(&code).unwrap();
    assert_eq!(json, "\"active\"");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"active\"";
    let code: Code = serde_json::from_str(json).unwrap();
    assert_eq!(code.as_str(), "active");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"active\"";
    let code: Code = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(code.as_str(), "active");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("active".to_string());
    let code: Code = serde_json::from_value(val).unwrap();
    assert_eq!(code.as_str(), "active");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Code>("\"\"").is_err());
    assert!(serde_json::from_str::<Code>("\" leading\"").is_err());
    assert!(serde_json::from_str::<Code>("\"double  space\"").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Code>("123").is_err());
    assert!(serde_json::from_str::<Code>("true").is_err());
    assert!(serde_json::from_str::<Code>("null").is_err());
    assert!(serde_json::from_str::<Code>("[]").is_err());
    assert!(serde_json::from_str::<Code>("{}").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let original = Code::new("round trip code").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Code = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_code_error_display_formatting() {
    assert_eq!(CodeError::Empty.to_string(), "code must not be empty");
    assert_eq!(
        CodeError::LeadingWhitespace.to_string(),
        "code must not start with whitespace"
    );
    assert_eq!(
        CodeError::TrailingWhitespace.to_string(),
        "code must not end with whitespace"
    );
    assert_eq!(
        CodeError::ConsecutiveWhitespace { index: 4 }.to_string(),
        "consecutive whitespace at byte index 4"
    );
    assert_eq!(
        CodeError::DisallowedWhitespaceCharacter {
            char: '\t',
            index: 3
        }
        .to_string(),
        "disallowed whitespace character '\\t' at byte index 3; only a single literal space is permitted"
    );
}
