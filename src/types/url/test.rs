use std::str::FromStr;

use super::*;

#[test]
fn test_valid_urls() {
    assert_eq!(Url::validate("https://example.org/index.html"), Ok(()));
    assert_eq!(Url::validate("mailto:someone@example.org"), Ok(()));
    assert_eq!(Url::validate("ftp://example.org/file.txt"), Ok(()));
}

#[test]
fn test_empty_is_valid() {
    assert_eq!(Url::validate(""), Ok(()));
}

#[test]
fn test_internal_whitespace_rejected() {
    assert_eq!(
        Url::validate("https://example .org"),
        Err(UrlError::InvalidCharacter {
            char: ' ',
            index: 15
        })
    );
}

#[test]
fn test_tab_and_newline_rejected() {
    assert_eq!(
        Url::validate("https://example.org\t/path"),
        Err(UrlError::InvalidCharacter {
            char: '\t',
            index: 19
        })
    );
    assert_eq!(
        Url::validate("https://example.org\n"),
        Err(UrlError::InvalidCharacter {
            char: '\n',
            index: 19
        })
    );
}

#[test]
fn test_leading_and_trailing_whitespace_rejected() {
    assert_eq!(
        Url::validate(" https://example.org"),
        Err(UrlError::InvalidCharacter {
            char: ' ',
            index: 0
        })
    );
    assert_eq!(
        Url::validate("https://example.org "),
        Err(UrlError::InvalidCharacter {
            char: ' ',
            index: 19
        })
    );
}

#[test]
fn test_error_message_output() {
    let result = Url::try_from("has a space");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value 'has a space' for type 'url': invalid whitespace character ' ' at byte index 3"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    let from_str = Url::new("https://example.org");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "https://example.org");

    let from_string = Url::new(String::from("https://example.org"));
    assert!(from_string.is_ok());

    let boxed_str: Box<str> = "https://example.org".into();
    let from_boxed = Url::new(boxed_str);
    assert!(from_boxed.is_ok());

    let invalid = Url::new("has a space");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let url = Url::new_unchecked("https://example.org");
    assert_eq!(url.as_str(), "https://example.org");
    assert_eq!(url.into_inner(), "https://example.org".to_string());

    let unvalidated = Url::new_unchecked("has a space");
    assert_eq!(unvalidated.as_str(), "has a space");
}

#[test]
fn test_from_str_and_display() {
    let url = Url::from_str("https://example.org").unwrap();
    assert_eq!(url.as_str(), "https://example.org");
    assert_eq!(format!("{url}"), "https://example.org");

    assert!(Url::from_str("has a space").is_err());
}

#[test]
fn test_from_url_into_string() {
    let url = Url::new("https://example.org").unwrap();
    let s: String = url.into();
    assert_eq!(s, "https://example.org");
}

#[test]
fn test_ordering_and_equality() {
    let a = Url::new_unchecked("https://a.org");
    let b = Url::new_unchecked("https://b.org");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    let url = Url::new("https://example.org").unwrap();
    let json = serde_json::to_string(&url).unwrap();
    assert_eq!(json, "\"https://example.org\"");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"https://example.org\"";
    let url: Url = serde_json::from_str(json).unwrap();
    assert_eq!(url.as_str(), "https://example.org");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"https://example.org\"";
    let url: Url = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(url.as_str(), "https://example.org");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("https://example.org".to_string());
    let url: Url = serde_json::from_value(val).unwrap();
    assert_eq!(url.as_str(), "https://example.org");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_empty_string_is_valid() {
    let url: Url = serde_json::from_str("\"\"").unwrap();
    assert_eq!(url.as_str(), "");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Url>("\"has a space\"").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Url>("123").is_err());
    assert!(serde_json::from_str::<Url>("true").is_err());
    assert!(serde_json::from_str::<Url>("null").is_err());
    assert!(serde_json::from_str::<Url>("[]").is_err());
    assert!(serde_json::from_str::<Url>("{}").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let original = Url::new("https://example.org/fhir/Patient/123").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Url = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_url_error_display_formatting() {
    let err = UrlError::InvalidCharacter {
        char: ' ',
        index: 3,
    };
    assert_eq!(
        err.to_string(),
        "invalid whitespace character ' ' at byte index 3"
    );
}
