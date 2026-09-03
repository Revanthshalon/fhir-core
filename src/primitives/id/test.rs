use std::str::FromStr;

use super::*;

#[test]
fn test_valid_ids() {
    assert_eq!(Id::validate("a"), Ok(()));
    assert_eq!(Id::validate("patient-1234"), Ok(()));
    assert_eq!(Id::validate("Observation.1"), Ok(()));
    assert_eq!(Id::validate("ABC-def-123.456"), Ok(()));
    assert_eq!(Id::validate("f7c40e2b-1a4e-4b8b-9c3d-2a6e5f8d9c1a"), Ok(()));
    assert_eq!(Id::validate("0123456789"), Ok(()));
}

#[test]
fn test_empty() {
    assert_eq!(Id::validate(""), Err(IdError::Empty));
}

#[test]
fn test_invalid_characters() {
    assert_eq!(
        Id::validate("has a space"),
        Err(IdError::InvalidCharacter {
            char: ' ',
            index: 3
        })
    );
    assert_eq!(
        Id::validate("under_score"),
        Err(IdError::InvalidCharacter {
            char: '_',
            index: 5
        })
    );
    assert_eq!(
        Id::validate("slash/id"),
        Err(IdError::InvalidCharacter {
            char: '/',
            index: 5
        })
    );
    assert_eq!(
        Id::validate("emoji🚀"),
        Err(IdError::InvalidCharacter {
            char: '🚀',
            index: 5
        })
    );
    assert_eq!(
        Id::validate("unicode-é"),
        Err(IdError::InvalidCharacter {
            char: 'é',
            index: 8
        })
    );
}

#[test]
fn test_max_length() {
    // Exactly at MAX_LENGTH (64 chars)
    let max_id = "a".repeat(Id::MAX_LENGTH);
    assert_eq!(Id::validate(&max_id), Ok(()));

    // Exceeding MAX_LENGTH by 1 char
    let over_max = "a".repeat(Id::MAX_LENGTH + 1);
    assert_eq!(
        Id::validate(&over_max),
        Err(IdError::ExceedsMaxLength {
            count: Id::MAX_LENGTH + 1,
            max: Id::MAX_LENGTH,
        })
    );

    // Well over MAX_LENGTH
    let way_over = "a".repeat(Id::MAX_LENGTH + 50);
    assert_eq!(
        Id::validate(&way_over),
        Err(IdError::ExceedsMaxLength {
            count: Id::MAX_LENGTH + 50,
            max: Id::MAX_LENGTH,
        })
    );
}

#[test]
fn test_error_message_output() {
    let result = Id::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'id': id must not be empty"
    );

    let invalid_char = Id::try_from("bad id");
    assert!(invalid_char.is_err());
    assert_eq!(
        invalid_char.unwrap_err().to_string(),
        "invalid value 'bad id' for type 'id': invalid character ' ' at byte index 3"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    // &str
    let from_str = Id::new("patient-1234");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "patient-1234");

    // String
    let from_string = Id::new(String::from("Observation.1"));
    assert!(from_string.is_ok());
    assert_eq!(from_string.unwrap().as_str(), "Observation.1");

    // Box<str>
    let boxed_str: Box<str> = "boxed-id".into();
    let from_boxed = Id::new(boxed_str);
    assert!(from_boxed.is_ok());
    assert_eq!(from_boxed.unwrap().as_str(), "boxed-id");

    // Invalid input
    let invalid = Id::new("has a space");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let id = Id::new_unchecked("trusted-id");
    assert_eq!(id.as_str(), "trusted-id");
    assert_eq!(id.into_inner(), "trusted-id".to_string());

    // Unchecked accepts even values that would fail validation
    let unvalidated = Id::new_unchecked("has a space");
    assert_eq!(unvalidated.as_str(), "has a space");
}

#[test]
fn test_from_str_and_display() {
    let id = Id::from_str("sample-id").unwrap();
    assert_eq!(id.as_str(), "sample-id");
    assert_eq!(format!("{id}"), "sample-id");

    assert!(Id::from_str("").is_err());
}

#[test]
fn test_from_id_into_string() {
    let id = Id::new("patient-1234").unwrap();
    let s: String = id.into();
    assert_eq!(s, "patient-1234");
}

#[test]
fn test_ordering_and_equality() {
    let a = Id::new_unchecked("a");
    let b = Id::new_unchecked("b");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[test]
fn test_serde_serialization() {
    let id = Id::new("patient-1234").unwrap();
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "\"patient-1234\"");
}

#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"patient-1234\"";
    let id: Id = serde_json::from_str(json).unwrap();
    assert_eq!(id.as_str(), "patient-1234");
}

#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"patient-1234\"";
    let id: Id = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(id.as_str(), "patient-1234");
}

#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("patient-1234".to_string());
    let id: Id = serde_json::from_value(val).unwrap();
    assert_eq!(id.as_str(), "patient-1234");
}

#[test]
fn test_serde_deserialization_invalid_value() {
    // Empty string
    assert!(serde_json::from_str::<Id>("\"\"").is_err());
    // Contains a space
    assert!(serde_json::from_str::<Id>("\"has a space\"").is_err());
    // Exceeds max length
    let over_max = format!("\"{}\"", "a".repeat(Id::MAX_LENGTH + 1));
    assert!(serde_json::from_str::<Id>(&over_max).is_err());
}

#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Id>("123").is_err());
    assert!(serde_json::from_str::<Id>("true").is_err());
    assert!(serde_json::from_str::<Id>("null").is_err());
    assert!(serde_json::from_str::<Id>("[]").is_err());
    assert!(serde_json::from_str::<Id>("{}").is_err());
}

#[test]
fn test_serde_roundtrip() {
    let original = Id::new("Roundtrip-Id.123").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Id = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_id_error_display_formatting() {
    let err = IdError::ExceedsMaxLength { count: 65, max: 64 };
    assert_eq!(
        err.to_string(),
        "character count (65) exceeds maximum allowed (64)"
    );
}

#[test]
fn test_id_boundary_characters() {
    // All allowed character classes at the boundaries
    assert!(Id::validate("A").is_ok());
    assert!(Id::validate("Z").is_ok());
    assert!(Id::validate("a").is_ok());
    assert!(Id::validate("z").is_ok());
    assert!(Id::validate("0").is_ok());
    assert!(Id::validate("9").is_ok());
    assert!(Id::validate("-").is_ok());
    assert!(Id::validate(".").is_ok());

    // Adjacent disallowed ASCII characters
    assert!(Id::validate("/").is_err()); // just before '0'..'9' ASCII-wise, disallowed
    assert!(Id::validate(":").is_err()); // just after '9', disallowed
    assert!(Id::validate("@").is_err()); // just before 'A', disallowed
    assert!(Id::validate("[").is_err()); // just after 'Z', disallowed
    assert!(Id::validate("`").is_err()); // just before 'a', disallowed
    assert!(Id::validate("{").is_err()); // just after 'z', disallowed
}
