use std::str::FromStr;

use super::*;

#[test]
fn test_valid_oids() {
    assert_eq!(Oid::validate("urn:oid:1.2.3.4"), Ok(()));
    assert_eq!(Oid::validate("urn:oid:2.999.1"), Ok(()));
    assert_eq!(Oid::validate("urn:oid:0.0"), Ok(()));
    assert_eq!(Oid::validate("urn:oid:1.0"), Ok(()));
    assert_eq!(Oid::validate("urn:oid:2.16.840.1.113883.3.1"), Ok(()));
}

#[test]
fn test_empty() {
    assert_eq!(Oid::validate(""), Err(OidError::Empty));
}

#[test]
fn test_missing_prefix() {
    assert_eq!(Oid::validate("1.2.3.4"), Err(OidError::MissingPrefix));
    assert_eq!(Oid::validate("oid:1.2.3.4"), Err(OidError::MissingPrefix));
}

#[test]
fn test_missing_root_arc() {
    assert_eq!(Oid::validate("urn:oid:"), Err(OidError::MissingRootArc));
}

#[test]
fn test_invalid_root_arc() {
    assert_eq!(
        Oid::validate("urn:oid:3.1"),
        Err(OidError::InvalidRootArc { found: '3' })
    );
    assert_eq!(
        Oid::validate("urn:oid:9.1"),
        Err(OidError::InvalidRootArc { found: '9' })
    );
}

#[test]
fn test_missing_arc() {
    assert_eq!(Oid::validate("urn:oid:1"), Err(OidError::MissingArc));
}

#[test]
fn test_invalid_arc() {
    assert_eq!(
        Oid::validate("urn:oid:1."),
        Err(OidError::InvalidArc {
            found: String::new()
        })
    );
    assert_eq!(
        Oid::validate("urn:oid:1..2"),
        Err(OidError::InvalidArc {
            found: String::new()
        })
    );
}

#[test]
fn test_leading_zero_in_arc() {
    assert_eq!(
        Oid::validate("urn:oid:1.01"),
        Err(OidError::LeadingZeroInArc {
            found: "01".to_owned()
        })
    );
    assert_eq!(
        Oid::validate("urn:oid:1.2.007"),
        Err(OidError::LeadingZeroInArc {
            found: "007".to_owned()
        })
    );
}

#[test]
fn test_unexpected_character() {
    assert_eq!(
        Oid::validate("urn:oid:15"),
        Err(OidError::UnexpectedCharacter {
            char: '5',
            index: 9
        })
    );
    assert_eq!(
        Oid::validate("urn:oid:1.2x"),
        Err(OidError::UnexpectedCharacter {
            char: 'x',
            index: 11
        })
    );
}

#[test]
fn test_error_message_output() {
    let result = Oid::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'oid': oid must not be empty"
    );

    let missing_prefix = Oid::try_from("1.2.3.4");
    assert!(missing_prefix.is_err());
    assert_eq!(
        missing_prefix.unwrap_err().to_string(),
        "invalid value '1.2.3.4' for type 'oid': oid must start with 'urn:oid:'"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    let from_str = Oid::new("urn:oid:1.2.3");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "urn:oid:1.2.3");

    let from_string = Oid::new(String::from("urn:oid:1.2.3"));
    assert!(from_string.is_ok());

    let boxed_str: Box<str> = "urn:oid:1.2.3".into();
    let from_boxed = Oid::new(boxed_str);
    assert!(from_boxed.is_ok());

    let invalid = Oid::new("1.2.3.4");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let oid = Oid::new_unchecked("urn:oid:1.2.3");
    assert_eq!(oid.as_str(), "urn:oid:1.2.3");
    assert_eq!(oid.into_inner(), "urn:oid:1.2.3".to_string());

    let unvalidated = Oid::new_unchecked("not-an-oid");
    assert_eq!(unvalidated.as_str(), "not-an-oid");
}

#[test]
fn test_from_str_and_display() {
    let oid = Oid::from_str("urn:oid:1.2.3").unwrap();
    assert_eq!(oid.as_str(), "urn:oid:1.2.3");
    assert_eq!(format!("{oid}"), "urn:oid:1.2.3");

    assert!(Oid::from_str("").is_err());
}

#[test]
fn test_from_oid_into_string() {
    let oid = Oid::new("urn:oid:1.2.3").unwrap();
    let s: String = oid.into();
    assert_eq!(s, "urn:oid:1.2.3");
}

#[test]
fn test_ordering_and_equality() {
    let a = Oid::new_unchecked("urn:oid:1.2.3");
    let b = Oid::new_unchecked("urn:oid:1.2.4");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    let oid = Oid::new("urn:oid:1.2.3").unwrap();
    let json = serde_json::to_string(&oid).unwrap();
    assert_eq!(json, "\"urn:oid:1.2.3\"");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"urn:oid:1.2.3\"";
    let oid: Oid = serde_json::from_str(json).unwrap();
    assert_eq!(oid.as_str(), "urn:oid:1.2.3");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"urn:oid:1.2.3\"";
    let oid: Oid = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(oid.as_str(), "urn:oid:1.2.3");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("urn:oid:1.2.3".to_string());
    let oid: Oid = serde_json::from_value(val).unwrap();
    assert_eq!(oid.as_str(), "urn:oid:1.2.3");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Oid>("\"\"").is_err());
    assert!(serde_json::from_str::<Oid>("\"1.2.3.4\"").is_err());
    assert!(serde_json::from_str::<Oid>("\"urn:oid:1.01\"").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Oid>("123").is_err());
    assert!(serde_json::from_str::<Oid>("true").is_err());
    assert!(serde_json::from_str::<Oid>("null").is_err());
    assert!(serde_json::from_str::<Oid>("[]").is_err());
    assert!(serde_json::from_str::<Oid>("{}").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let original = Oid::new("urn:oid:2.16.840.1.113883.3.1").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Oid = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_oid_error_display_formatting() {
    let err = OidError::LeadingZeroInArc {
        found: "01".to_owned(),
    };
    assert_eq!(err.to_string(), "arc '01' must not have a leading zero");
}
