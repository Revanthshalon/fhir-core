use std::str::FromStr;

use super::*;

#[test]
fn test_validate_zero() {
    assert_eq!(Integer::validate("0"), Ok(0));
}

#[test]
fn test_validate_positive_without_sign() {
    assert_eq!(Integer::validate("1"), Ok(1));
    assert_eq!(Integer::validate("42"), Ok(42));
    assert_eq!(Integer::validate("123456789"), Ok(123_456_789));
}

#[test]
fn test_validate_positive_with_explicit_sign() {
    assert_eq!(Integer::validate("+1"), Ok(1));
    assert_eq!(Integer::validate("+42"), Ok(42));
}

#[test]
fn test_validate_negative() {
    assert_eq!(Integer::validate("-1"), Ok(-1));
    assert_eq!(Integer::validate("-42"), Ok(-42));
    assert_eq!(Integer::validate("-123456789"), Ok(-123_456_789));
}

#[test]
fn test_validate_range_boundaries() {
    assert_eq!(Integer::validate("2147483647"), Ok(i32::MAX));
    assert_eq!(Integer::validate("+2147483647"), Ok(i32::MAX));
    assert_eq!(Integer::validate("-2147483648"), Ok(i32::MIN));
}

#[test]
fn test_validate_single_digits() {
    for d in 0..=9u8 {
        let s = d.to_string();
        assert_eq!(Integer::validate(&s), Ok(d as i32));
    }
}

// ---------------------------------------------------------------------
// validate(): negative paths
// ---------------------------------------------------------------------

#[test]
fn test_validate_empty_string() {
    assert_eq!(Integer::validate(""), Err(IntegerError::Empty));
}

#[test]
fn test_validate_lone_sign() {
    assert_eq!(Integer::validate("+"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("-"), Err(IntegerError::InvalidFormat));
}

#[test]
fn test_validate_signed_zero_rejected() {
    // Per the FHIR regex `[0]|[-+]?[1-9][0-9]*`, only a bare "0" matches — a signed zero
    // matches neither alternative and must be rejected.
    assert_eq!(Integer::validate("+0"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("-0"), Err(IntegerError::InvalidFormat));
    assert!(Integer::try_from("+0").is_err());
    assert!(Integer::try_from("-0").is_err());
    assert!(serde_json::from_str::<Integer>("42").is_ok()); // sanity: normal path unaffected
}

#[test]
fn test_validate_float_values_rejected() {
    assert_eq!(Integer::validate("1.0"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("1.5"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("-1.5"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("0.0"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate(".5"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("5."), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("1e10"), Err(IntegerError::InvalidFormat));
    assert_eq!(
        Integer::validate("2147483647.0"),
        Err(IntegerError::InvalidFormat)
    );
}

#[test]
fn test_validate_negative_extreme_underflow() {
    // Comfortably beyond i32::MIN.
    assert_eq!(
        Integer::validate("-999999999999999999999999"),
        Err(IntegerError::OutOfRange)
    );
    // i64::MIN, still out of range for a 32-bit integer.
    assert_eq!(
        Integer::validate("-9223372036854775808"),
        Err(IntegerError::OutOfRange)
    );
    // One below i32::MIN.
    assert_eq!(
        Integer::validate("-2147483649"),
        Err(IntegerError::OutOfRange)
    );
}

#[test]
fn test_validate_whitespace_around_signed_integers() {
    assert_eq!(Integer::validate(" +1"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("+1 "), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate(" -1"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("-1 "), Err(IntegerError::InvalidFormat));
    assert_eq!(
        Integer::validate("\t+1\t"),
        Err(IntegerError::InvalidFormat)
    );
    assert_eq!(
        Integer::validate("\n-1\n"),
        Err(IntegerError::InvalidFormat)
    );
    assert_eq!(Integer::validate("+ 1"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("- 1"), Err(IntegerError::InvalidFormat));
}

#[test]
fn test_validate_control_characters() {
    assert_eq!(Integer::validate("1\0"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("\x001"), Err(IntegerError::InvalidFormat));
    assert_eq!(
        Integer::validate("+1\u{0008}"),
        Err(IntegerError::InvalidFormat)
    );
    assert_eq!(
        Integer::validate("-\u{001B}1"),
        Err(IntegerError::InvalidFormat)
    );
}

#[test]
fn test_validate_mixed_double_signs() {
    assert_eq!(Integer::validate("++1"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("--1"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("+-1"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("-+1"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("+-+1"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("-+-1"), Err(IntegerError::InvalidFormat));
}

#[test]
fn test_validate_leading_zero() {
    assert_eq!(Integer::validate("01"), Err(IntegerError::LeadingZero));
    assert_eq!(Integer::validate("007"), Err(IntegerError::LeadingZero));
    assert_eq!(Integer::validate("+01"), Err(IntegerError::LeadingZero));
    assert_eq!(Integer::validate("-01"), Err(IntegerError::LeadingZero));
    assert_eq!(Integer::validate("00"), Err(IntegerError::LeadingZero));
}

#[test]
fn test_validate_non_digit_characters() {
    assert_eq!(Integer::validate("abc"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("12a"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("a12"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("1.0"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("1e5"), Err(IntegerError::InvalidFormat));
}

#[test]
fn test_validate_internal_whitespace() {
    assert_eq!(Integer::validate("1 2"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate(" 12"), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate("12 "), Err(IntegerError::InvalidFormat));
    assert_eq!(Integer::validate(" "), Err(IntegerError::InvalidFormat));
}

#[test]
fn test_validate_out_of_range() {
    assert_eq!(
        Integer::validate("2147483648"),
        Err(IntegerError::OutOfRange)
    );
    assert_eq!(
        Integer::validate("+2147483648"),
        Err(IntegerError::OutOfRange)
    );
    assert_eq!(
        Integer::validate("-2147483649"),
        Err(IntegerError::OutOfRange)
    );
    assert_eq!(
        Integer::validate("99999999999999999999"),
        Err(IntegerError::OutOfRange)
    );
}

#[test]
fn test_validate_unicode_digits_rejected() {
    // Full-width Unicode digit (U+FF11 "１") is not an ASCII digit.
    assert_eq!(
        Integer::validate("\u{FF11}"),
        Err(IntegerError::InvalidFormat)
    );
}

// ---------------------------------------------------------------------
// new / as_i32 / into_inner / new_unchecked
// ---------------------------------------------------------------------

#[test]
fn test_new_and_accessors() {
    let i = Integer::new(42);
    assert_eq!(i.as_i32(), 42);
    assert_eq!(i.into_inner(), 42);
}

#[test]
fn test_new_covers_full_i32_range() {
    assert_eq!(Integer::new(i32::MIN).as_i32(), i32::MIN);
    assert_eq!(Integer::new(i32::MAX).as_i32(), i32::MAX);
    assert_eq!(Integer::new(0).as_i32(), 0);
}

#[test]
fn test_as_i32_mut() {
    let mut i = Integer::new(10);
    *i.as_i32_mut() += 5;
    assert_eq!(i.as_i32(), 15);

    *i.as_i32_mut() = i32::MAX;
    assert_eq!(i, Integer::new(i32::MAX));
}

#[test]
fn test_new_unchecked() {
    let i = Integer::new_unchecked(123);
    assert_eq!(i.as_i32(), 123);
}

#[test]
fn test_default() {
    assert_eq!(Integer::default(), Integer::new(0));
}

#[test]
fn test_min_max_constants() {
    assert_eq!(Integer::MIN, i32::MIN);
    assert_eq!(Integer::MAX, i32::MAX);
}

// ---------------------------------------------------------------------
// From / TryFrom conversions
// ---------------------------------------------------------------------

#[test]
fn test_from_i32() {
    let i: Integer = 7.into();
    assert_eq!(i.as_i32(), 7);
}

#[test]
fn test_into_i32() {
    let i = Integer::new(9);
    let raw: i32 = i.into();
    assert_eq!(raw, 9);
}

#[test]
fn test_try_from_str_valid() {
    assert_eq!(Integer::try_from("42"), Ok(Integer::new(42)));
    assert_eq!(Integer::try_from("-42"), Ok(Integer::new(-42)));
    assert_eq!(Integer::try_from("0"), Ok(Integer::new(0)));
}

#[test]
fn test_try_from_str_invalid() {
    assert!(Integer::try_from("007").is_err());
    assert!(Integer::try_from("").is_err());
    assert!(Integer::try_from("abc").is_err());
    assert!(Integer::try_from("99999999999").is_err());
}

#[test]
fn test_try_from_string_valid() {
    let s = String::from("100");
    assert_eq!(Integer::try_from(s), Ok(Integer::new(100)));
}

#[test]
fn test_try_from_string_invalid() {
    let s = String::from("0100");
    assert!(Integer::try_from(s).is_err());
}

#[test]
fn test_from_str_and_display() {
    let i = Integer::from_str("123").unwrap();
    assert_eq!(i.as_i32(), 123);
    assert_eq!(format!("{i}"), "123");

    let neg = Integer::from_str("-5").unwrap();
    assert_eq!(format!("{neg}"), "-5");

    assert!(Integer::from_str("bad").is_err());
}

#[test]
fn test_error_message_output() {
    let err = Integer::try_from("007").unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid value '007' for type 'integer': non-zero integer must not have a leading zero"
    );

    let empty_err = Integer::try_from("").unwrap_err();
    assert_eq!(
        empty_err.to_string(),
        "invalid value '' for type 'integer': integer string must not be empty"
    );

    let range_err = Integer::try_from("9999999999").unwrap_err();
    assert_eq!(
        range_err.to_string(),
        "invalid value '9999999999' for type 'integer': value is out of range for a 32-bit signed integer (-2147483648..2147483647)"
    );
}

#[test]
fn test_integer_error_display_variants() {
    assert_eq!(
        IntegerError::Empty.to_string(),
        "integer string must not be empty"
    );
    assert_eq!(
        IntegerError::LeadingZero.to_string(),
        "non-zero integer must not have a leading zero"
    );
    assert_eq!(
        IntegerError::InvalidFormat.to_string(),
        "not a valid integer representation"
    );
    assert_eq!(
        IntegerError::OutOfRange.to_string(),
        "value is out of range for a 32-bit signed integer (-2147483648..2147483647)"
    );
}

// ---------------------------------------------------------------------
// Ordering / Hash / Eq
// ---------------------------------------------------------------------

#[test]
fn test_ordering() {
    assert!(Integer::new(1) < Integer::new(2));
    assert!(Integer::new(-1) < Integer::new(0));
    assert_eq!(Integer::new(5), Integer::new(5));
}

#[test]
fn test_serde_serialization() {
    let i = Integer::new(42);
    let json = serde_json::to_string(&i).unwrap();
    assert_eq!(json, "42");

    let neg = Integer::new(-42);
    assert_eq!(serde_json::to_string(&neg).unwrap(), "-42");
}

#[test]
fn test_serde_deserialization_from_str() {
    let i: Integer = serde_json::from_str("42").unwrap();
    assert_eq!(i.as_i32(), 42);

    let neg: Integer = serde_json::from_str("-42").unwrap();
    assert_eq!(neg.as_i32(), -42);

    let zero: Integer = serde_json::from_str("0").unwrap();
    assert_eq!(zero.as_i32(), 0);
}

#[test]
fn test_serde_deserialization_boundaries() {
    let max: Integer = serde_json::from_str("2147483647").unwrap();
    assert_eq!(max.as_i32(), i32::MAX);

    let min: Integer = serde_json::from_str("-2147483648").unwrap();
    assert_eq!(min.as_i32(), i32::MIN);
}

#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"7";
    let i: Integer = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(i.as_i32(), 7);
}

#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::Number(serde_json::Number::from(15));
    let i: Integer = serde_json::from_value(val).unwrap();
    assert_eq!(i.as_i32(), 15);
}

#[test]
fn test_serde_roundtrip() {
    for value in [i32::MIN, -1, 0, 1, i32::MAX] {
        let original = Integer::new(value);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Integer = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}

#[test]
fn test_serde_deserialization_out_of_range() {
    // Exceeds i32::MAX
    assert!(serde_json::from_str::<Integer>("2147483648").is_err());
    // Below i32::MIN
    assert!(serde_json::from_str::<Integer>("-2147483649").is_err());
    // Wildly out of range
    assert!(serde_json::from_str::<Integer>("99999999999999999999").is_err());
}

#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<Integer>("\"42\"").is_err());
    assert!(serde_json::from_str::<Integer>("true").is_err());
    assert!(serde_json::from_str::<Integer>("null").is_err());
    assert!(serde_json::from_str::<Integer>("[]").is_err());
    assert!(serde_json::from_str::<Integer>("{}").is_err());
}

#[test]
fn test_serde_deserialization_non_integer_number() {
    // FHIR integer is a JSON number without a decimal point; 1.5 cannot fit in i32.
    assert!(serde_json::from_str::<Integer>("1.5").is_err());
}

#[test]
fn test_serde_deserialization_malformed_json() {
    assert!(serde_json::from_str::<Integer>("").is_err());
    assert!(serde_json::from_str::<Integer>("abc").is_err());
}
