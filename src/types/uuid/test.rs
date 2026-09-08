use std::str::FromStr;

use super::*;

const VALID: &str = "urn:uuid:c757873d-ec9a-4326-a141-556f43239520";

#[test]
fn test_valid_uuid() {
    assert_eq!(Uuid::validate(VALID), Ok(()));
    assert_eq!(
        Uuid::validate("urn:uuid:00000000-0000-0000-0000-000000000000"),
        Ok(())
    );
    assert_eq!(
        Uuid::validate("urn:uuid:ffffffff-ffff-ffff-ffff-ffffffffffff"),
        Ok(())
    );
}

#[test]
fn test_empty() {
    assert_eq!(Uuid::validate(""), Err(UuidError::Empty));
}

#[test]
fn test_missing_prefix() {
    assert_eq!(
        Uuid::validate("c757873d-ec9a-4326-a141-556f43239520"),
        Err(UuidError::MissingPrefix)
    );
}

#[test]
fn test_uppercase_rejected() {
    assert_eq!(
        Uuid::validate("urn:uuid:C757873D-ec9a-4326-a141-556f43239520"),
        Err(UuidError::InvalidCharacter {
            char: 'C',
            index: 9
        })
    );
}

#[test]
fn test_invalid_length_too_short() {
    assert_eq!(
        Uuid::validate("urn:uuid:c757873d-ec9a-4326-a141"),
        Err(UuidError::InvalidLength { found: 23 })
    );
}

#[test]
fn test_invalid_length_too_long() {
    assert_eq!(
        Uuid::validate("urn:uuid:c757873d-ec9a-4326-a141-556f432395200"),
        Err(UuidError::InvalidLength { found: 37 })
    );
}

#[test]
fn test_missing_hyphen() {
    assert_eq!(
        Uuid::validate("urn:uuid:c757873dxec9a-4326-a141-556f43239520"),
        Err(UuidError::InvalidCharacter {
            char: 'x',
            index: 17
        })
    );
}

#[test]
fn test_invalid_hex_character() {
    assert_eq!(
        Uuid::validate("urn:uuid:g757873d-ec9a-4326-a141-556f43239520"),
        Err(UuidError::InvalidCharacter {
            char: 'g',
            index: 9
        })
    );
}

#[test]
fn test_error_message_output() {
    let result = Uuid::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'uuid': uuid must not be empty"
    );

    let missing_prefix = Uuid::try_from("c757873d-ec9a-4326-a141-556f43239520");
    assert!(missing_prefix.is_err());
    assert_eq!(
        missing_prefix.unwrap_err().to_string(),
        "invalid value 'c757873d-ec9a-4326-a141-556f43239520' for type 'uuid': uuid must start with 'urn:uuid:'"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    let from_str = Uuid::new(VALID);
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), VALID);

    let from_string = Uuid::new(String::from(VALID));
    assert!(from_string.is_ok());

    let boxed_str: Box<str> = VALID.into();
    let from_boxed = Uuid::new(boxed_str);
    assert!(from_boxed.is_ok());

    let invalid = Uuid::new("not-a-uuid");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let uuid = Uuid::new_unchecked(VALID);
    assert_eq!(uuid.as_str(), VALID);
    assert_eq!(uuid.into_inner(), VALID.to_string());

    let unvalidated = Uuid::new_unchecked("not-a-uuid");
    assert_eq!(unvalidated.as_str(), "not-a-uuid");
}

#[test]
fn test_from_str_and_display() {
    let uuid = Uuid::from_str(VALID).unwrap();
    assert_eq!(uuid.as_str(), VALID);
    assert_eq!(format!("{uuid}"), VALID);

    assert!(Uuid::from_str("").is_err());
}

#[test]
fn test_from_uuid_into_string() {
    let uuid = Uuid::new(VALID).unwrap();
    let s: String = uuid.into();
    assert_eq!(s, VALID);
}

#[test]
fn test_ordering_and_equality() {
    let a = Uuid::new_unchecked("urn:uuid:00000000-0000-0000-0000-000000000000");
    let b = Uuid::new_unchecked("urn:uuid:ffffffff-ffff-ffff-ffff-ffffffffffff");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[test]
fn test_serde_serialization() {
    let uuid = Uuid::new(VALID).unwrap();
    let json = serde_json::to_string(&uuid).unwrap();
    assert_eq!(json, format!("\"{VALID}\""));
}

#[test]
fn test_serde_deserialization_from_str() {
    let json = format!("\"{VALID}\"");
    let uuid: Uuid = serde_json::from_str(&json).unwrap();
    assert_eq!(uuid.as_str(), VALID);
}

#[test]
fn test_serde_deserialization_from_reader() {
    let json = format!("\"{VALID}\"");
    let uuid: Uuid = serde_json::from_reader(json.as_bytes()).unwrap();
    assert_eq!(uuid.as_str(), VALID);
}

#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String(VALID.to_string());
    let uuid: Uuid = serde_json::from_value(val).unwrap();
    assert_eq!(uuid.as_str(), VALID);
}

#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Uuid>("\"\"").is_err());
    assert!(serde_json::from_str::<Uuid>("\"c757873d-ec9a-4326-a141-556f43239520\"").is_err());
    assert!(
        serde_json::from_str::<Uuid>("\"urn:uuid:C757873D-EC9A-4326-A141-556F43239520\"").is_err()
    );
}

#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Uuid>("123").is_err());
    assert!(serde_json::from_str::<Uuid>("true").is_err());
    assert!(serde_json::from_str::<Uuid>("null").is_err());
    assert!(serde_json::from_str::<Uuid>("[]").is_err());
    assert!(serde_json::from_str::<Uuid>("{}").is_err());
}

#[test]
fn test_serde_roundtrip() {
    let original = Uuid::new(VALID).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Uuid = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_uuid_error_display_formatting() {
    let err = UuidError::InvalidLength { found: 10 };
    assert_eq!(
        err.to_string(),
        "expected 36 characters after 'urn:uuid:' (8-4-4-4-12 hex groups), found 10"
    );
}
