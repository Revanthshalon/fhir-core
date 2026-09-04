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
        Base64Binary::validate("===="),
        Err(Base64Error::InvalidPadding { count: 4 })
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

#[test]
fn test_into_inner_and_from_into_string() {
    let b64 = Base64Binary::new("TWFu").unwrap();
    assert_eq!(b64.clone().into_inner(), "TWFu");

    let s: String = b64.into();
    assert_eq!(s, "TWFu");
}

#[test]
fn test_from_str() {
    use std::str::FromStr;

    let b64 = Base64Binary::from_str("TQ==").unwrap();
    assert_eq!(b64.as_str(), "TQ==");
    assert!(Base64Binary::from_str("invalid").is_err());
}

#[test]
fn test_insufficient_data_before_padding() {
    assert_eq!(
        Base64Binary::validate("A==="),
        Err(Base64Error::InsufficientDataBeforePadding)
    );
    assert_eq!(
        Base64Binary::validate("TWFuA==="),
        Err(Base64Error::InsufficientDataBeforePadding)
    );
    assert_eq!(
        Base64Binary::validate("===="),
        Err(Base64Error::InvalidPadding { count: 4 })
    );
}

#[test]
fn test_base64_error_display_formatting() {
    assert_eq!(
        Base64Error::InvalidLength(5).to_string(),
        "length (5) is not a multiple of 4"
    );
    assert_eq!(
        Base64Error::InvalidPadding { count: 4 }.to_string(),
        "invalid padding: found 4 '=' characters (max allowed is 2)"
    );
    assert_eq!(
        Base64Error::UnexpectedPadding { index: 1 }.to_string(),
        "unexpected padding character '=' at index 1"
    );
    assert_eq!(
        Base64Error::InsufficientDataBeforePadding.to_string(),
        "invalid padded block: requires at least 2 data characters before padding"
    );
}

#[test]
fn test_base64_negative_whitespace_rejection() {
    assert!(Base64Binary::validate(" TWFu").is_err());
    assert!(Base64Binary::validate("TWFu ").is_err());
    assert!(Base64Binary::validate("TW Fu").is_err());
    assert!(Base64Binary::validate("TW\nFu").is_err());
    assert!(Base64Binary::validate("TW\tFu").is_err());
    assert!(Base64Binary::validate("TW\rFu").is_err());
}

#[test]
fn test_base64_negative_url_safe_and_symbols() {
    // URL-safe '-' and '_' are not allowed in standard RFC 4648 Base64
    assert!(Base64Binary::validate("TW-u").is_err());
    assert!(Base64Binary::validate("TW_u").is_err());
    assert!(Base64Binary::validate("TW.u").is_err());
    assert!(Base64Binary::validate("TW!u").is_err());
    assert!(Base64Binary::validate("TW$u").is_err());
}

#[test]
fn test_base64_negative_internal_padding() {
    // Padding in the middle of a stream
    assert_eq!(
        Base64Binary::validate("TQ==TQ=="),
        Err(Base64Error::UnexpectedPadding { index: 2 })
    );
    assert_eq!(
        Base64Binary::validate("TW=u"),
        Err(Base64Error::UnexpectedPadding { index: 2 })
    );
    assert_eq!(
        Base64Binary::validate("=TWF"),
        Err(Base64Error::UnexpectedPadding { index: 0 })
    );
}

#[test]
fn test_base64_negative_non_ascii() {
    assert!(Base64Binary::validate("TWé=").is_err());
    assert!(Base64Binary::validate("TW🦀=").is_err());
}

#[test]
fn test_base64_negative_lengths() {
    assert_eq!(
        Base64Binary::validate("A"),
        Err(Base64Error::InvalidLength(1))
    );
    assert_eq!(
        Base64Binary::validate("AB"),
        Err(Base64Error::InvalidLength(2))
    );
    assert_eq!(
        Base64Binary::validate("ABC"),
        Err(Base64Error::InvalidLength(3))
    );
    assert_eq!(
        Base64Binary::validate("ABCDE"),
        Err(Base64Error::InvalidLength(5))
    );
    assert_eq!(
        Base64Binary::validate("ABCDEF"),
        Err(Base64Error::InvalidLength(6))
    );
    assert_eq!(
        Base64Binary::validate("ABCDEFG"),
        Err(Base64Error::InvalidLength(7))
    );
}
