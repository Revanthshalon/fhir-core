use std::str::FromStr;

use super::*;

// ---------------------------------------------------------------------
// validate(): happy paths
// ---------------------------------------------------------------------

#[test]
fn test_validate_zero() {
    assert_eq!(Integer64::validate("0"), Ok(0));
}

#[test]
fn test_validate_positive_without_sign() {
    assert_eq!(Integer64::validate("1"), Ok(1));
    assert_eq!(Integer64::validate("42"), Ok(42));
    assert_eq!(Integer64::validate("123456789"), Ok(123_456_789));
}

#[test]
fn test_validate_positive_with_explicit_sign() {
    assert_eq!(Integer64::validate("+1"), Ok(1));
    assert_eq!(Integer64::validate("+42"), Ok(42));
}

#[test]
fn test_validate_negative() {
    assert_eq!(Integer64::validate("-1"), Ok(-1));
    assert_eq!(Integer64::validate("-42"), Ok(-42));
    assert_eq!(Integer64::validate("-123456789"), Ok(-123_456_789));
}

#[test]
fn test_validate_range_boundaries() {
    assert_eq!(Integer64::validate("9223372036854775807"), Ok(i64::MAX));
    assert_eq!(Integer64::validate("+9223372036854775807"), Ok(i64::MAX));
    assert_eq!(Integer64::validate("-9223372036854775808"), Ok(i64::MIN));
}

#[test]
fn test_validate_single_digits() {
    for d in 0..=9u8 {
        let s = d.to_string();
        assert_eq!(Integer64::validate(&s), Ok(d as i64));
    }
}

#[test]
fn test_validate_large_values_beyond_i32_range() {
    // Values that would overflow i32 but are valid for i64 — the whole point of this type.
    assert_eq!(Integer64::validate("2147483648"), Ok(2_147_483_648));
    assert_eq!(Integer64::validate("-2147483649"), Ok(-2_147_483_649));
    assert_eq!(
        Integer64::validate("123456789012345"),
        Ok(123_456_789_012_345)
    );
}

// ---------------------------------------------------------------------
// validate(): negative paths
// ---------------------------------------------------------------------

#[test]
fn test_validate_empty_string() {
    assert_eq!(Integer64::validate(""), Err(Integer64Error::Empty));
}

#[test]
fn test_validate_lone_sign() {
    assert_eq!(Integer64::validate("+"), Err(Integer64Error::InvalidFormat));
    assert_eq!(Integer64::validate("-"), Err(Integer64Error::InvalidFormat));
}

#[test]
fn test_validate_signed_zero_rejected() {
    // Per the FHIR regex `[0]|[-+]?[1-9][0-9]*`, only a bare "0" matches — a signed zero
    // matches neither alternative and must be rejected.
    assert_eq!(
        Integer64::validate("+0"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("-0"),
        Err(Integer64Error::InvalidFormat)
    );
    assert!(Integer64::try_from("+0").is_err());
    assert!(Integer64::try_from("-0").is_err());
}

#[test]
fn test_validate_float_values_rejected() {
    assert_eq!(
        Integer64::validate("1.0"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("1.5"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("-1.5"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate(".5"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("5."),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("1e10"),
        Err(Integer64Error::InvalidFormat)
    );
}

#[test]
fn test_validate_negative_extreme_underflow() {
    // Comfortably beyond i64::MIN.
    assert_eq!(
        Integer64::validate("-999999999999999999999999999999"),
        Err(Integer64Error::OutOfRange)
    );
    // One below i64::MIN.
    assert_eq!(
        Integer64::validate("-9223372036854775809"),
        Err(Integer64Error::OutOfRange)
    );
}

#[test]
fn test_validate_positive_extreme_overflow() {
    // One above i64::MAX.
    assert_eq!(
        Integer64::validate("9223372036854775808"),
        Err(Integer64Error::OutOfRange)
    );
    assert_eq!(
        Integer64::validate("999999999999999999999999999999"),
        Err(Integer64Error::OutOfRange)
    );
}

#[test]
fn test_validate_whitespace_around_signed_integers() {
    assert_eq!(
        Integer64::validate(" +1"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("+1 "),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate(" -1"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("-1 "),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("\t+1\t"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("\n-1\n"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("+ 1"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("- 1"),
        Err(Integer64Error::InvalidFormat)
    );
}

#[test]
fn test_validate_control_characters() {
    assert_eq!(
        Integer64::validate("1\0"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("\x001"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("+1\u{0008}"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("-\u{001B}1"),
        Err(Integer64Error::InvalidFormat)
    );
}

#[test]
fn test_validate_mixed_double_signs() {
    assert_eq!(
        Integer64::validate("++1"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("--1"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("+-1"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("-+1"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("+-+1"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("-+-1"),
        Err(Integer64Error::InvalidFormat)
    );
}

#[test]
fn test_validate_leading_zero() {
    assert_eq!(Integer64::validate("01"), Err(Integer64Error::LeadingZero));
    assert_eq!(Integer64::validate("007"), Err(Integer64Error::LeadingZero));
    assert_eq!(Integer64::validate("+01"), Err(Integer64Error::LeadingZero));
    assert_eq!(Integer64::validate("-01"), Err(Integer64Error::LeadingZero));
    assert_eq!(Integer64::validate("00"), Err(Integer64Error::LeadingZero));
}

#[test]
fn test_validate_non_digit_characters() {
    assert_eq!(
        Integer64::validate("abc"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("12a"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("a12"),
        Err(Integer64Error::InvalidFormat)
    );
}

#[test]
fn test_validate_internal_whitespace() {
    assert_eq!(
        Integer64::validate("1 2"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate(" 12"),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(
        Integer64::validate("12 "),
        Err(Integer64Error::InvalidFormat)
    );
    assert_eq!(Integer64::validate(" "), Err(Integer64Error::InvalidFormat));
}

#[test]
fn test_validate_unicode_digits_rejected() {
    // Full-width Unicode digit (U+FF11 "1") is not an ASCII digit.
    assert_eq!(
        Integer64::validate("\u{FF11}"),
        Err(Integer64Error::InvalidFormat)
    );
}

// ---------------------------------------------------------------------
// new / as_i64 / as_i64_mut / into_inner / new_unchecked
// ---------------------------------------------------------------------

#[test]
fn test_new_and_accessors() {
    let i = Integer64::new(42);
    assert_eq!(i.as_i64(), 42);
    assert_eq!(i.into_inner(), 42);
}

#[test]
fn test_new_covers_full_i64_range() {
    assert_eq!(Integer64::new(i64::MIN).as_i64(), i64::MIN);
    assert_eq!(Integer64::new(i64::MAX).as_i64(), i64::MAX);
    assert_eq!(Integer64::new(0).as_i64(), 0);
}

#[test]
fn test_as_i64_mut() {
    let mut i = Integer64::new(10);
    *i.as_i64_mut() += 5;
    assert_eq!(i.as_i64(), 15);

    *i.as_i64_mut() = i64::MAX;
    assert_eq!(i, Integer64::new(i64::MAX));
}

#[test]
fn test_new_unchecked() {
    let i = Integer64::new_unchecked(123);
    assert_eq!(i.as_i64(), 123);
}

#[test]
fn test_default() {
    assert_eq!(Integer64::default(), Integer64::new(0));
}

#[test]
fn test_min_max_constants() {
    assert_eq!(Integer64::MIN, i64::MIN);
    assert_eq!(Integer64::MAX, i64::MAX);
}

// ---------------------------------------------------------------------
// From / TryFrom conversions
// ---------------------------------------------------------------------

#[test]
fn test_from_i64() {
    let i: Integer64 = 7.into();
    assert_eq!(i.as_i64(), 7);
}

#[test]
fn test_into_i64() {
    let i = Integer64::new(9);
    let raw: i64 = i.into();
    assert_eq!(raw, 9);
}

#[test]
fn test_try_from_str_valid() {
    assert_eq!(Integer64::try_from("42"), Ok(Integer64::new(42)));
    assert_eq!(Integer64::try_from("-42"), Ok(Integer64::new(-42)));
    assert_eq!(Integer64::try_from("0"), Ok(Integer64::new(0)));
}

#[test]
fn test_try_from_str_invalid() {
    assert!(Integer64::try_from("007").is_err());
    assert!(Integer64::try_from("").is_err());
    assert!(Integer64::try_from("abc").is_err());
    assert!(Integer64::try_from("99999999999999999999").is_err());
}

#[test]
fn test_try_from_string_valid() {
    let s = String::from("100");
    assert_eq!(Integer64::try_from(s), Ok(Integer64::new(100)));
}

#[test]
fn test_try_from_string_invalid() {
    let s = String::from("0100");
    assert!(Integer64::try_from(s).is_err());
}

#[test]
fn test_from_str_and_display() {
    let i = Integer64::from_str("123").unwrap();
    assert_eq!(i.as_i64(), 123);
    assert_eq!(format!("{i}"), "123");

    let neg = Integer64::from_str("-5").unwrap();
    assert_eq!(format!("{neg}"), "-5");

    assert!(Integer64::from_str("bad").is_err());
}

#[test]
fn test_error_message_output() {
    let err = Integer64::try_from("007").unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid value '007' for type 'integer64': non-zero integer64 must not have a leading zero"
    );

    let empty_err = Integer64::try_from("").unwrap_err();
    assert_eq!(
        empty_err.to_string(),
        "invalid value '' for type 'integer64': integer64 string must not be empty"
    );

    let range_err = Integer64::try_from("99999999999999999999").unwrap_err();
    assert_eq!(
        range_err.to_string(),
        "invalid value '99999999999999999999' for type 'integer64': value is out of range for a 64-bit signed integer (-9223372036854775808..9223372036854775807)"
    );
}

#[test]
fn test_integer64_error_display_variants() {
    assert_eq!(
        Integer64Error::Empty.to_string(),
        "integer64 string must not be empty"
    );
    assert_eq!(
        Integer64Error::LeadingZero.to_string(),
        "non-zero integer64 must not have a leading zero"
    );
    assert_eq!(
        Integer64Error::InvalidFormat.to_string(),
        "not a valid integer64 representation"
    );
    assert_eq!(
        Integer64Error::OutOfRange.to_string(),
        "value is out of range for a 64-bit signed integer (-9223372036854775808..9223372036854775807)"
    );
}

// ---------------------------------------------------------------------
// Ordering / Hash / Eq
// ---------------------------------------------------------------------

#[test]
fn test_ordering() {
    assert!(Integer64::new(1) < Integer64::new(2));
    assert!(Integer64::new(-1) < Integer64::new(0));
    assert_eq!(Integer64::new(5), Integer64::new(5));
}

// ---------------------------------------------------------------------
// serde: happy paths — integer64 is transmitted as a JSON STRING, not a number
// ---------------------------------------------------------------------

#[test]
fn test_serde_serialization_is_json_string() {
    let i = Integer64::new(42);
    let json = serde_json::to_string(&i).unwrap();
    assert_eq!(json, "\"42\"");

    let neg = Integer64::new(-42);
    assert_eq!(serde_json::to_string(&neg).unwrap(), "\"-42\"");
}

#[test]
fn test_serde_serialization_preserves_precision_at_extremes() {
    let max = Integer64::new(i64::MAX);
    assert_eq!(
        serde_json::to_string(&max).unwrap(),
        "\"9223372036854775807\""
    );

    let min = Integer64::new(i64::MIN);
    assert_eq!(
        serde_json::to_string(&min).unwrap(),
        "\"-9223372036854775808\""
    );
}

#[test]
fn test_serde_deserialization_from_json_string() {
    let i: Integer64 = serde_json::from_str("\"42\"").unwrap();
    assert_eq!(i.as_i64(), 42);

    let neg: Integer64 = serde_json::from_str("\"-42\"").unwrap();
    assert_eq!(neg.as_i64(), -42);

    let zero: Integer64 = serde_json::from_str("\"0\"").unwrap();
    assert_eq!(zero.as_i64(), 0);
}

#[test]
fn test_serde_deserialization_boundaries() {
    let max: Integer64 = serde_json::from_str("\"9223372036854775807\"").unwrap();
    assert_eq!(max.as_i64(), i64::MAX);

    let min: Integer64 = serde_json::from_str("\"-9223372036854775808\"").unwrap();
    assert_eq!(min.as_i64(), i64::MIN);
}

#[test]
fn test_serde_deserialization_from_reader() {
    let json = b"\"7\"";
    let i: Integer64 = serde_json::from_reader(&json[..]).unwrap();
    assert_eq!(i.as_i64(), 7);
}

#[test]
fn test_serde_deserialization_from_value() {
    let val = serde_json::Value::String("15".to_string());
    let i: Integer64 = serde_json::from_value(val).unwrap();
    assert_eq!(i.as_i64(), 15);
}

#[test]
fn test_serde_roundtrip() {
    for value in [i64::MIN, -1, 0, 1, i64::MAX] {
        let original = Integer64::new(value);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Integer64 = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}

// ---------------------------------------------------------------------
// serde: negative paths
// ---------------------------------------------------------------------

#[test]
fn test_serde_deserialization_out_of_range() {
    // Exceeds i64::MAX
    assert!(serde_json::from_str::<Integer64>("\"9223372036854775808\"").is_err());
    // Below i64::MIN
    assert!(serde_json::from_str::<Integer64>("\"-9223372036854775809\"").is_err());
    // Wildly out of range
    assert!(serde_json::from_str::<Integer64>("\"999999999999999999999999999999\"").is_err());
}

#[test]
fn test_serde_deserialization_invalid_value() {
    assert!(serde_json::from_str::<Integer64>("\"\"").is_err());
    assert!(serde_json::from_str::<Integer64>("\"007\"").is_err());
    assert!(serde_json::from_str::<Integer64>("\"abc\"").is_err());
    assert!(serde_json::from_str::<Integer64>("\"1.5\"").is_err());
}

#[test]
fn test_serde_deserialization_invalid_type() {
    // A bare JSON number is invalid — integer64 SHALL be a JSON string.
    assert!(serde_json::from_str::<Integer64>("42").is_err());
    assert!(serde_json::from_str::<Integer64>("true").is_err());
    assert!(serde_json::from_str::<Integer64>("null").is_err());
    assert!(serde_json::from_str::<Integer64>("[]").is_err());
    assert!(serde_json::from_str::<Integer64>("{}").is_err());
}

#[test]
fn test_serde_deserialization_malformed_json() {
    assert!(serde_json::from_str::<Integer64>("").is_err());
    assert!(serde_json::from_str::<Integer64>("abc").is_err());
}
