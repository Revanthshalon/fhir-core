use std::str::FromStr;

use super::*;

#[test]
fn test_valid_values() {
    assert_eq!(UnsignedInt::validate("0"), Ok(0));
    assert_eq!(UnsignedInt::validate("1"), Ok(1));
    assert_eq!(UnsignedInt::validate("42"), Ok(42));
    assert_eq!(UnsignedInt::validate("2147483647"), Ok(2_147_483_647));
}

#[test]
fn test_empty() {
    assert_eq!(UnsignedInt::validate(""), Err(UnsignedIntError::Empty));
}

#[test]
fn test_negative_rejected() {
    assert_eq!(
        UnsignedInt::validate("-1"),
        Err(UnsignedIntError::InvalidFormat)
    );
    assert_eq!(
        UnsignedInt::validate("-0"),
        Err(UnsignedIntError::InvalidFormat)
    );
}

#[test]
fn test_leading_plus_rejected() {
    assert_eq!(
        UnsignedInt::validate("+0"),
        Err(UnsignedIntError::InvalidFormat)
    );
    assert_eq!(
        UnsignedInt::validate("+1"),
        Err(UnsignedIntError::InvalidFormat)
    );
}

#[test]
fn test_leading_zero() {
    assert_eq!(
        UnsignedInt::validate("01"),
        Err(UnsignedIntError::LeadingZero)
    );
    assert_eq!(
        UnsignedInt::validate("007"),
        Err(UnsignedIntError::LeadingZero)
    );
}

#[test]
fn test_out_of_range() {
    assert_eq!(
        UnsignedInt::validate("2147483648"),
        Err(UnsignedIntError::OutOfRange)
    );
    assert_eq!(
        UnsignedInt::validate("99999999999999999999"),
        Err(UnsignedIntError::OutOfRange)
    );
}

#[test]
fn test_non_digit_characters() {
    assert_eq!(
        UnsignedInt::validate("12a"),
        Err(UnsignedIntError::InvalidFormat)
    );
    assert_eq!(
        UnsignedInt::validate("1.5"),
        Err(UnsignedIntError::InvalidFormat)
    );
}

#[test]
fn test_error_message_output() {
    let result = UnsignedInt::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'unsignedInt': unsignedInt string must not be empty"
    );

    let leading_zero = UnsignedInt::try_from("01");
    assert!(leading_zero.is_err());
    assert_eq!(
        leading_zero.unwrap_err().to_string(),
        "invalid value '01' for type 'unsignedInt': non-zero unsignedInt must not have a leading zero"
    );
}

#[test]
fn test_new_from_u32() {
    assert_eq!(UnsignedInt::new(0).unwrap().as_u32(), 0);
    assert_eq!(UnsignedInt::new(42).unwrap().as_u32(), 42);
    assert!(UnsignedInt::new(u32::MAX).is_err());
}

#[test]
fn test_try_from_u32_error_message() {
    let too_big = UnsignedInt::try_from(3_000_000_000u32);
    assert!(too_big.is_err());
    assert_eq!(
        too_big.unwrap_err().to_string(),
        "invalid value '3000000000' for type 'unsignedInt': value exceeds the maximum of 2147483647"
    );
}

#[test]
fn test_new_unchecked() {
    let u = UnsignedInt::new_unchecked(42);
    assert_eq!(u.as_u32(), 42);
    assert_eq!(u.into_inner(), 42);
}

#[test]
fn test_from_str_and_display() {
    let u = UnsignedInt::from_str("42").unwrap();
    assert_eq!(u.as_u32(), 42);
    assert_eq!(format!("{u}"), "42");

    assert!(UnsignedInt::from_str("-1").is_err());
}

#[test]
fn test_from_unsigned_int_into_u32() {
    let u = UnsignedInt::new(42).unwrap();
    let n: u32 = u.into();
    assert_eq!(n, 42);
}

#[test]
fn test_new_with_string_types() {
    let from_str = UnsignedInt::try_from("42");
    assert!(from_str.is_ok());

    let from_string = UnsignedInt::try_from(String::from("42"));
    assert!(from_string.is_ok());
}

#[test]
fn test_ordering_and_equality() {
    let a = UnsignedInt::new_unchecked(0);
    let b = UnsignedInt::new_unchecked(1);
    assert!(a < b);
    assert_eq!(a, a);
    assert_ne!(a, b);
}

#[test]
fn test_bounds_constants() {
    assert_eq!(UnsignedInt::MIN, 0);
    assert_eq!(UnsignedInt::MAX, 2_147_483_647);
}

#[test]
fn test_serde_serialization() {
    let u = UnsignedInt::new(42).unwrap();
    let json = serde_json::to_string(&u).unwrap();
    assert_eq!(json, "42");
}

#[test]
fn test_serde_deserialization_from_str() {
    let json = "42";
    let u: UnsignedInt = serde_json::from_str(json).unwrap();
    assert_eq!(u.as_u32(), 42);
}

#[test]
fn test_serde_deserialization_boundaries() {
    let max: UnsignedInt = serde_json::from_str("2147483647").unwrap();
    assert_eq!(max.as_u32(), 2_147_483_647);

    let min: UnsignedInt = serde_json::from_str("0").unwrap();
    assert_eq!(min.as_u32(), 0);
}

#[test]
fn test_serde_deserialization_out_of_range() {
    assert!(serde_json::from_str::<UnsignedInt>("2147483648").is_err());
    assert!(serde_json::from_str::<UnsignedInt>("99999999999").is_err());
}

#[test]
fn test_serde_deserialization_negative_rejected() {
    assert!(serde_json::from_str::<UnsignedInt>("-1").is_err());
}

#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<UnsignedInt>("\"42\"").is_err());
    assert!(serde_json::from_str::<UnsignedInt>("true").is_err());
    assert!(serde_json::from_str::<UnsignedInt>("null").is_err());
    assert!(serde_json::from_str::<UnsignedInt>("[]").is_err());
    assert!(serde_json::from_str::<UnsignedInt>("{}").is_err());
}

#[test]
fn test_serde_deserialization_non_integer_number() {
    assert!(serde_json::from_str::<UnsignedInt>("1.5").is_err());
}

#[test]
fn test_serde_roundtrip() {
    let original = UnsignedInt::new(12345).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: UnsignedInt = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_unsigned_int_error_display_formatting() {
    let err = UnsignedIntError::OutOfRange;
    assert_eq!(err.to_string(), "value exceeds the maximum of 2147483647");
}
