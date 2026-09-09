use std::str::FromStr;

use super::*;

#[test]
fn test_valid_canonicals() {
    assert_eq!(
        Canonical::validate("http://hl7.org/fhir/StructureDefinition/Patient"),
        Ok(())
    );
    assert_eq!(Canonical::validate("uri|1.0#frag"), Ok(()));
    assert_eq!(
        Canonical::validate("http://hl7.org/fhir/StructureDefinition/Patient|5.0.0"),
        Ok(())
    );
}

#[test]
fn test_empty_is_valid() {
    assert_eq!(Canonical::validate(""), Ok(()));
}

#[test]
fn test_internal_whitespace_rejected() {
    assert_eq!(
        Canonical::validate("http://example .org"),
        Err(CanonicalError::InvalidCharacter {
            char: ' ',
            index: 14
        })
    );
}

#[test]
fn test_tab_and_newline_rejected() {
    assert_eq!(
        Canonical::validate("http://example.org\t"),
        Err(CanonicalError::InvalidCharacter {
            char: '\t',
            index: 18
        })
    );
    assert_eq!(
        Canonical::validate("http://example.org\n"),
        Err(CanonicalError::InvalidCharacter {
            char: '\n',
            index: 18
        })
    );
}

#[test]
fn test_leading_and_trailing_whitespace_rejected() {
    assert_eq!(
        Canonical::validate(" http://example.org"),
        Err(CanonicalError::InvalidCharacter {
            char: ' ',
            index: 0
        })
    );
    assert_eq!(
        Canonical::validate("http://example.org "),
        Err(CanonicalError::InvalidCharacter {
            char: ' ',
            index: 18
        })
    );
}

#[test]
fn test_error_message_output() {
    let result = Canonical::try_from("has a space");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value 'has a space' for type 'canonical': invalid whitespace character ' ' at byte index 3"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    let from_str = Canonical::new("http://example.org");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "http://example.org");

    let from_string = Canonical::new(String::from("http://example.org"));
    assert!(from_string.is_ok());

    let boxed_str: Box<str> = "http://example.org".into();
    let from_boxed = Canonical::new(boxed_str);
    assert!(from_boxed.is_ok());

    let invalid = Canonical::new("has a space");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let canonical = Canonical::new_unchecked("http://example.org|1.0");
    assert_eq!(canonical.as_str(), "http://example.org|1.0");
    assert_eq!(canonical.into_inner(), "http://example.org|1.0".to_string());

    let unvalidated = Canonical::new_unchecked("has a space");
    assert_eq!(unvalidated.as_str(), "has a space");
}

#[test]
fn test_from_str_and_display() {
    let canonical = Canonical::from_str("http://example.org").unwrap();
    assert_eq!(canonical.as_str(), "http://example.org");
    assert_eq!(format!("{canonical}"), "http://example.org");

    assert!(Canonical::from_str("has a space").is_err());
}

#[test]
fn test_from_canonical_into_string() {
    let canonical = Canonical::new("http://example.org").unwrap();
    let s: String = canonical.into();
    assert_eq!(s, "http://example.org");
}

#[test]
fn test_ordering_and_equality() {
    let a = Canonical::new_unchecked("http://a.org");
    let b = Canonical::new_unchecked("http://b.org");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    let canonical = Canonical::new("http://example.org").unwrap();
    let json = serde_json::to_string(&canonical).unwrap();
    assert_eq!(json, "\"http://example.org\"");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"http://example.org\"";
    let canonical: Canonical = serde_json::from_str(json).unwrap();
    assert_eq!(canonical.as_str(), "http://example.org");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"http://example.org\"";
    let canonical: Canonical = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(canonical.as_str(), "http://example.org");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("http://example.org".to_string());
    let canonical: Canonical = serde_json::from_value(val).unwrap();
    assert_eq!(canonical.as_str(), "http://example.org");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_empty_string_is_valid() {
    let canonical: Canonical = serde_json::from_str("\"\"").unwrap();
    assert_eq!(canonical.as_str(), "");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Canonical>("\"has a space\"").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Canonical>("123").is_err());
    assert!(serde_json::from_str::<Canonical>("true").is_err());
    assert!(serde_json::from_str::<Canonical>("null").is_err());
    assert!(serde_json::from_str::<Canonical>("[]").is_err());
    assert!(serde_json::from_str::<Canonical>("{}").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let original = Canonical::new("http://example.org/StructureDefinition/Foo|1.0").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Canonical = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_canonical_error_display_formatting() {
    let err = CanonicalError::InvalidCharacter {
        char: ' ',
        index: 3,
    };
    assert_eq!(
        err.to_string(),
        "invalid whitespace character ' ' at byte index 3"
    );
}
