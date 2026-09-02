use super::*;

#[test]
fn test_valid_base64() {
    assert_eq!(Base64Binary::validate(""), Ok(()));
    assert_eq!(Base64Binary::validate("TWFu"), Ok(()));
    assert_eq!(Base64Binary::validate("TWE="), Ok(()));
    assert_eq!(Base64Binary::validate("TQ=="), Ok(()));
    assert_eq!(Base64Binary::validate("AAAA/123+xyz"), Ok(()));
}

#[test]
fn test_invalid_length() {
    assert_eq!(
        Base64Binary::validate("TWF"),
        Err(Base64Error::InvalidLength(3))
    );
}

#[test]
fn test_invalid_character() {
    assert_eq!(
        Base64Binary::validate("TW@u"),
        Err(Base64Error::InvalidCharacter {
            char: '@',
            index: 2
        })
    );
}

#[test]
fn test_unexpected_padding() {
    assert_eq!(
        Base64Binary::validate("T=Fu"),
        Err(Base64Error::UnexpectedPadding { index: 1 })
    );
}

#[test]
fn test_too_much_padding() {
    assert_eq!(
        Base64Binary::validate("T==="),
        Err(Base64Error::InvalidPadding { count: 3 })
    );
}

#[test]
fn test_error_message_output() {
    let result = Base64Binary::try_from("TW@u");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert_eq!(
        err_msg,
        "invalid value 'TW@u' for type 'base64Binary': invalid character '@' at index 2"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    // &str
    let from_str = Base64Binary::new("TWFu");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "TWFu");

    // String
    let from_string = Base64Binary::new(String::from("TWE="));
    assert!(from_string.is_ok());
    assert_eq!(from_string.unwrap().as_str(), "TWE=");

    // Box<str>
    let boxed_str: Box<str> = "TQ==".into();
    let from_boxed = Base64Binary::new(boxed_str);
    assert!(from_boxed.is_ok());
    assert_eq!(from_boxed.unwrap().as_str(), "TQ==");

    // Invalid input
    let invalid = Base64Binary::new(String::from("invalid"));
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let b64 = Base64Binary::new_unchecked("TWFu");
    assert_eq!(b64.as_str(), "TWFu");

    let b64_string = Base64Binary::new_unchecked(String::from("TQ=="));
    assert_eq!(b64_string.as_str(), "TQ==");

    // Unchecked accepts even strings that would otherwise fail validation
    let b64_unvalidated = Base64Binary::new_unchecked("not_base64!");
    assert_eq!(b64_unvalidated.as_str(), "not_base64!");
}

#[test]
fn test_serde_serialization() {
    let b64 = Base64Binary::new("TWFu").unwrap();
    let json = serde_json::to_string(&b64).unwrap();
    assert_eq!(json, "\"TWFu\"");
}

#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"TWFu\"";
    let b64: Base64Binary = serde_json::from_str(json).unwrap();
    assert_eq!(b64.as_str(), "TWFu");
}

#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"TWFu\"";
    let b64: Base64Binary = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(b64.as_str(), "TWFu");
}

#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("TWFu".to_string());
    let b64: Base64Binary = serde_json::from_value(val).unwrap();
    assert_eq!(b64.as_str(), "TWFu");
}

#[test]
fn test_serde_deserialization_escaped() {
    let json = r#""\u0054\u0057\u0046\u0075""#;
    let b64: Base64Binary = serde_json::from_str(json).unwrap();
    assert_eq!(b64.as_str(), "TWFu");
}

#[test]
fn test_serde_deserialization_invalid_value() {
    let json = "\"invalid\"";
    let res: Result<Base64Binary, _> = serde_json::from_str(json);
    assert!(res.is_err());
}

#[test]
fn test_serde_deserialization_invalid_type() {
    let json = "12345";
    let res: Result<Base64Binary, _> = serde_json::from_str(json);
    assert!(res.is_err());
}

#[test]
fn test_serde_roundtrip() {
    let original = Base64Binary::new("TQ==").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Base64Binary = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_serde_empty_string() {
    let original = Base64Binary::new("").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    assert_eq!(json, "\"\"");
    let deserialized: Base64Binary = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.as_str(), "");
}
