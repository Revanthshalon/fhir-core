use std::str::FromStr;

use super::*;

#[test]
fn test_valid_uris() {
    assert_eq!(Uri::validate("http://example.org"), Ok(()));
    assert_eq!(Uri::validate("urn:uuid:123"), Ok(()));
    assert_eq!(Uri::validate("/relative/path"), Ok(()));
    assert_eq!(Uri::validate("urn:oid:1.2.3.4"), Ok(()));
}

#[test]
fn test_empty_is_valid() {
    assert_eq!(Uri::validate(""), Ok(()));
}

#[test]
fn test_internal_whitespace_rejected() {
    assert_eq!(
        Uri::validate("http://example .org"),
        Err(UriError::InvalidCharacter {
            char: ' ',
            index: 14
        })
    );
}

#[test]
fn test_tab_and_newline_rejected() {
    assert_eq!(
        Uri::validate("http://example.org\t/path"),
        Err(UriError::InvalidCharacter {
            char: '\t',
            index: 18
        })
    );
    assert_eq!(
        Uri::validate("http://example.org\n"),
        Err(UriError::InvalidCharacter {
            char: '\n',
            index: 18
        })
    );
}

#[test]
fn test_leading_and_trailing_whitespace_rejected() {
    assert_eq!(
        Uri::validate(" http://example.org"),
        Err(UriError::InvalidCharacter {
            char: ' ',
            index: 0
        })
    );
    assert_eq!(
        Uri::validate("http://example.org "),
        Err(UriError::InvalidCharacter {
            char: ' ',
            index: 18
        })
    );
}

#[test]
fn test_error_message_output() {
    let result = Uri::try_from("has a space");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value 'has a space' for type 'uri': invalid whitespace character ' ' at byte index 3"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    let from_str = Uri::new("http://example.org");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "http://example.org");

    let from_string = Uri::new(String::from("http://example.org"));
    assert!(from_string.is_ok());

    let boxed_str: Box<str> = "http://example.org".into();
    let from_boxed = Uri::new(boxed_str);
    assert!(from_boxed.is_ok());

    let invalid = Uri::new("has a space");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let uri = Uri::new_unchecked("http://example.org");
    assert_eq!(uri.as_str(), "http://example.org");
    assert_eq!(uri.into_inner(), "http://example.org".to_string());

    let unvalidated = Uri::new_unchecked("has a space");
    assert_eq!(unvalidated.as_str(), "has a space");
}

#[test]
fn test_from_str_and_display() {
    let uri = Uri::from_str("http://example.org").unwrap();
    assert_eq!(uri.as_str(), "http://example.org");
    assert_eq!(format!("{uri}"), "http://example.org");

    assert!(Uri::from_str("has a space").is_err());
}

#[test]
fn test_from_uri_into_string() {
    let uri = Uri::new("http://example.org").unwrap();
    let s: String = uri.into();
    assert_eq!(s, "http://example.org");
}

#[test]
fn test_ordering_and_equality() {
    let a = Uri::new_unchecked("http://a.org");
    let b = Uri::new_unchecked("http://b.org");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    let uri = Uri::new("http://example.org").unwrap();
    let json = serde_json::to_string(&uri).unwrap();
    assert_eq!(json, "\"http://example.org\"");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"http://example.org\"";
    let uri: Uri = serde_json::from_str(json).unwrap();
    assert_eq!(uri.as_str(), "http://example.org");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"http://example.org\"";
    let uri: Uri = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(uri.as_str(), "http://example.org");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("http://example.org".to_string());
    let uri: Uri = serde_json::from_value(val).unwrap();
    assert_eq!(uri.as_str(), "http://example.org");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_empty_string_is_valid() {
    let uri: Uri = serde_json::from_str("\"\"").unwrap();
    assert_eq!(uri.as_str(), "");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Uri>("\"has a space\"").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Uri>("123").is_err());
    assert!(serde_json::from_str::<Uri>("true").is_err());
    assert!(serde_json::from_str::<Uri>("null").is_err());
    assert!(serde_json::from_str::<Uri>("[]").is_err());
    assert!(serde_json::from_str::<Uri>("{}").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let original = Uri::new("http://example.org/fhir/Patient/123").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Uri = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_uri_error_display_formatting() {
    let err = UriError::InvalidCharacter {
        char: ' ',
        index: 3,
    };
    assert_eq!(
        err.to_string(),
        "invalid whitespace character ' ' at byte index 3"
    );
}
