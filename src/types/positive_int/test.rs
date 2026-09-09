use std::str::FromStr;

use super::*;

#[test]
fn test_valid_values() {
    assert_eq!(PositiveInt::validate("1"), Ok(1));
    assert_eq!(PositiveInt::validate("42"), Ok(42));
    assert_eq!(PositiveInt::validate("2147483647"), Ok(2_147_483_647));
}

#[test]
fn test_empty() {
    assert_eq!(PositiveInt::validate(""), Err(PositiveIntError::Empty));
}

#[test]
fn test_zero_rejected() {
    assert_eq!(
        PositiveInt::validate("0"),
        Err(PositiveIntError::NotPositive)
    );
}

#[test]
fn test_negative_rejected() {
    assert_eq!(
        PositiveInt::validate("-1"),
        Err(PositiveIntError::InvalidFormat)
    );
}

#[test]
fn test_leading_plus_rejected() {
    assert_eq!(
        PositiveInt::validate("+1"),
        Err(PositiveIntError::InvalidFormat)
    );
}

#[test]
fn test_leading_zero() {
    assert_eq!(
        PositiveInt::validate("01"),
        Err(PositiveIntError::LeadingZero)
    );
    assert_eq!(
        PositiveInt::validate("007"),
        Err(PositiveIntError::LeadingZero)
    );
}

#[test]
fn test_out_of_range() {
    assert_eq!(
        PositiveInt::validate("2147483648"),
        Err(PositiveIntError::OutOfRange)
    );
    assert_eq!(
        PositiveInt::validate("99999999999999999999"),
        Err(PositiveIntError::OutOfRange)
    );
}

#[test]
fn test_non_digit_characters() {
    assert_eq!(
        PositiveInt::validate("12a"),
        Err(PositiveIntError::InvalidFormat)
    );
    assert_eq!(
        PositiveInt::validate("1.5"),
        Err(PositiveIntError::InvalidFormat)
    );
}

#[test]
fn test_error_message_output() {
    let result = PositiveInt::try_from("");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid value '' for type 'positiveInt': positiveInt string must not be empty"
    );

    let zero = PositiveInt::try_from("0");
    assert!(zero.is_err());
    assert_eq!(
        zero.unwrap_err().to_string(),
        "invalid value '0' for type 'positiveInt': positiveInt must not be 0"
    );
}

#[test]
fn test_new_from_u32() {
    assert_eq!(PositiveInt::new(42).unwrap().as_u32(), 42);
    assert!(PositiveInt::new(0).is_err());
    assert!(PositiveInt::new(u32::MAX).is_err());
}

#[test]
fn test_try_from_u32_error_messages() {
    let zero = PositiveInt::try_from(0u32);
    assert!(zero.is_err());
    assert_eq!(
        zero.unwrap_err().to_string(),
        "invalid value '0' for type 'positiveInt': positiveInt must not be 0"
    );

    let too_big = PositiveInt::try_from(3_000_000_000u32);
    assert!(too_big.is_err());
    assert_eq!(
        too_big.unwrap_err().to_string(),
        "invalid value '3000000000' for type 'positiveInt': value exceeds the maximum of 2147483647"
    );
}

#[test]
fn test_new_unchecked() {
    let p = PositiveInt::new_unchecked(42);
    assert_eq!(p.as_u32(), 42);
    assert_eq!(p.into_inner(), 42);
}

#[test]
fn test_from_str_and_display() {
    let p = PositiveInt::from_str("42").unwrap();
    assert_eq!(p.as_u32(), 42);
    assert_eq!(format!("{p}"), "42");

    assert!(PositiveInt::from_str("0").is_err());
}

#[test]
fn test_from_positive_int_into_u32() {
    let p = PositiveInt::new(42).unwrap();
    let n: u32 = p.into();
    assert_eq!(n, 42);
}

#[test]
fn test_new_with_string_types() {
    let from_str = PositiveInt::try_from("42");
    assert!(from_str.is_ok());

    let from_string = PositiveInt::try_from(String::from("42"));
    assert!(from_string.is_ok());
}

#[test]
fn test_ordering_and_equality() {
    let a = PositiveInt::new_unchecked(1);
    let b = PositiveInt::new_unchecked(2);
    assert!(a < b);
    assert_eq!(a, a);
    assert_ne!(a, b);
}

#[test]
fn test_bounds_constants() {
    assert_eq!(PositiveInt::MIN, 1);
    assert_eq!(PositiveInt::MAX, 2_147_483_647);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    let p = PositiveInt::new(42).unwrap();
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, "42");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_from_str() {
    let json = "42";
    let p: PositiveInt = serde_json::from_str(json).unwrap();
    assert_eq!(p.as_u32(), 42);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_boundaries() {
    let max: PositiveInt = serde_json::from_str("2147483647").unwrap();
    assert_eq!(max.as_u32(), 2_147_483_647);

    let min: PositiveInt = serde_json::from_str("1").unwrap();
    assert_eq!(min.as_u32(), 1);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_zero_rejected() {
    assert!(serde_json::from_str::<PositiveInt>("0").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_out_of_range() {
    assert!(serde_json::from_str::<PositiveInt>("2147483648").is_err());
    assert!(serde_json::from_str::<PositiveInt>("99999999999").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_negative_rejected() {
    assert!(serde_json::from_str::<PositiveInt>("-1").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_invalid_type() {
    assert!(serde_json::from_str::<PositiveInt>("\"42\"").is_err());
    assert!(serde_json::from_str::<PositiveInt>("true").is_err());
    assert!(serde_json::from_str::<PositiveInt>("null").is_err());
    assert!(serde_json::from_str::<PositiveInt>("[]").is_err());
    assert!(serde_json::from_str::<PositiveInt>("{}").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialization_non_integer_number() {
    assert!(serde_json::from_str::<PositiveInt>("1.5").is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let original = PositiveInt::new(12345).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PositiveInt = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_positive_int_error_display_formatting() {
    let err = PositiveIntError::OutOfRange;
    assert_eq!(err.to_string(), "value exceeds the maximum of 2147483647");
}
