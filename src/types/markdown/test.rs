use std::str::FromStr;

use super::*;

#[test]
fn test_valid_markdown() {
    assert_eq!(Markdown::validate("# Header\n\n**bold**"), Ok(()));
    assert_eq!(Markdown::validate("Simple text with 123!"), Ok(()));
    assert_eq!(
        Markdown::validate("Contains\ttab,\nnewline,\rand carriage return"),
        Ok(())
    );
    assert_eq!(Markdown::validate("Unicode: 🚀 é ñ 漢字 مرحبا 🌍"), Ok(()));
    assert_eq!(Markdown::validate("- item 1\n- item 2"), Ok(()));
}

#[test]
fn test_empty_and_whitespace_only() {
    assert_eq!(
        Markdown::validate(""),
        Err(MarkdownError::EmptyOrWhitespaceOnly)
    );
    assert_eq!(
        Markdown::validate("   "),
        Err(MarkdownError::EmptyOrWhitespaceOnly)
    );
    assert_eq!(
        Markdown::validate("\t\t"),
        Err(MarkdownError::EmptyOrWhitespaceOnly)
    );
    assert_eq!(
        Markdown::validate("\n\r\n"),
        Err(MarkdownError::EmptyOrWhitespaceOnly)
    );
}

#[test]
fn test_disallowed_control_characters() {
    assert_eq!(
        Markdown::validate("hello\0world"),
        Err(MarkdownError::DisallowedControlCharacter {
            char: '\0',
            index: 5
        })
    );
    assert_eq!(
        Markdown::validate("\u{0001}abc"),
        Err(MarkdownError::DisallowedControlCharacter {
            char: '\u{0001}',
            index: 0
        })
    );
    assert_eq!(
        Markdown::validate("esc\u{001B}key"),
        Err(MarkdownError::DisallowedControlCharacter {
            char: '\u{001B}',
            index: 3
        })
    );
}

#[test]
fn test_control_character_boundaries() {
    assert!(Markdown::validate("a\u{0008}b").is_err()); // Backspace disallowed
    assert!(Markdown::validate("a\u{0009}b").is_ok()); // Tab allowed
    assert!(Markdown::validate("a\u{000A}b").is_ok()); // Line Feed allowed
    assert!(Markdown::validate("a\u{000B}b").is_err()); // Vertical Tab disallowed
    assert!(Markdown::validate("a\u{000D}b").is_ok()); // Carriage Return allowed
    assert!(Markdown::validate("a\u{001F}b").is_err()); // Unit Separator disallowed
    assert!(Markdown::validate("a\u{0020}b").is_ok()); // Space allowed
}

#[test]
fn test_max_length() {
    let max_str = "a".repeat(Markdown::MAX_LENGTH);
    assert_eq!(Markdown::validate(&max_str), Ok(()));

    let over_max = "a".repeat(Markdown::MAX_LENGTH + 1);
    assert_eq!(
        Markdown::validate(&over_max),
        Err(MarkdownError::ExceedsMaxLength {
            count: Markdown::MAX_LENGTH + 1,
            max: Markdown::MAX_LENGTH,
        })
    );

    let emoji_str = "🦀".repeat(100_000);
    assert_eq!(Markdown::validate(&emoji_str), Ok(()));
}

#[test]
fn test_error_message_output() {
    let result = Markdown::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'markdown': markdown must contain non-whitespace content and cannot be empty"
    );

    let ctrl_err = Markdown::try_from("bad\0val");
    assert!(ctrl_err.is_err());
    assert_eq!(
        ctrl_err.unwrap_err().to_string(),
        "invalid value 'bad\0val' for type 'markdown': disallowed control character '\\0' (U+0000) at byte index 3"
    );
}

#[test]
fn test_new_with_various_into_string_types() {
    let from_str = Markdown::new("**bold**");
    assert!(from_str.is_ok());
    assert_eq!(from_str.unwrap().as_str(), "**bold**");

    let from_string = Markdown::new(String::from("# Title"));
    assert!(from_string.is_ok());
    assert_eq!(from_string.unwrap().as_str(), "# Title");

    let boxed_str: Box<str> = "_italic_".into();
    let from_boxed = Markdown::new(boxed_str);
    assert!(from_boxed.is_ok());
    assert_eq!(from_boxed.unwrap().as_str(), "_italic_");

    let invalid = Markdown::new("   ");
    assert!(invalid.is_err());
}

#[test]
fn test_new_unchecked() {
    let md = Markdown::new_unchecked("Valid Value");
    assert_eq!(md.as_str(), "Valid Value");
    assert_eq!(md.into_inner(), "Valid Value");

    let unvalidated = Markdown::new_unchecked("   ");
    assert_eq!(unvalidated.as_str(), "   ");
}

#[test]
fn test_from_str_and_display() {
    let md = Markdown::from_str("**bold**").unwrap();
    assert_eq!(md.as_str(), "**bold**");
    assert_eq!(format!("{md}"), "**bold**");

    assert!(Markdown::from_str("").is_err());
}

#[test]
fn test_from_markdown_into_string() {
    let md = Markdown::new("**bold**").unwrap();
    let s: String = md.into();
    assert_eq!(s, "**bold**");
}

#[test]
fn test_ordering_and_equality() {
    let a = Markdown::new_unchecked("a");
    let b = Markdown::new_unchecked("b");
    assert!(a < b);
    assert_eq!(a.clone(), a);
    assert_ne!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    let md = Markdown::new("**bold**").unwrap();
    let json = serde_json::to_string(&md).unwrap();
    assert_eq!(json, "\"**bold**\"");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_str() {
    let json = "\"**bold**\"";
    let md: Markdown = serde_json::from_str(json).unwrap();
    assert_eq!(md.as_str(), "**bold**");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"**bold**\"";
    let md: Markdown = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(md.as_str(), "**bold**");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("**bold**".to_string());
    let md: Markdown = serde_json::from_value(val).unwrap();
    assert_eq!(md.as_str(), "**bold**");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_escaped() {
    let json = r#""Hello""#;
    let md: Markdown = serde_json::from_str(json).unwrap();
    assert_eq!(md.as_str(), "Hello");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Markdown>("\"\"").is_err());
    assert!(serde_json::from_str::<Markdown>("\"   \"").is_err());
    assert!(serde_json::from_str::<Markdown>(r#""hello\u0000world""#).is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Markdown>("123").is_err());
    assert!(serde_json::from_str::<Markdown>("true").is_err());
    assert!(serde_json::from_str::<Markdown>("null").is_err());
    assert!(serde_json::from_str::<Markdown>("[]").is_err());
    assert!(serde_json::from_str::<Markdown>("{}").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let original = Markdown::new("# Title\n\nBody text with 🚀").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Markdown = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_markdown_error_display_formatting() {
    let err = MarkdownError::ExceedsMaxLength {
        count: 1_048_577,
        max: 1_048_576,
    };
    assert_eq!(
        err.to_string(),
        "character count (1048577) exceeds maximum allowed (1048576)"
    );
}
