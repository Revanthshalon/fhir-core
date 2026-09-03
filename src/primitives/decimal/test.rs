use std::str::FromStr;

use super::*;

// ---------------------------------------------------------------------
// validate(): happy paths
// ---------------------------------------------------------------------

#[test]
fn test_validate_zero() {
    assert_eq!(Decimal::validate("0"), Ok(()));
}

#[test]
fn test_validate_negative_zero_allowed() {
    // Unlike Integer (which rejects "-0"/"+0"), decimal's regex wraps the sign around the
    // whole alternation, so "-0" is syntactically valid here.
    assert_eq!(Decimal::validate("-0"), Ok(()));
    assert!(Decimal::try_from("-0").is_ok());
}

#[test]
fn test_validate_integer_values() {
    assert_eq!(Decimal::validate("1"), Ok(()));
    assert_eq!(Decimal::validate("42"), Ok(()));
    assert_eq!(Decimal::validate("-42"), Ok(()));
    assert_eq!(Decimal::validate("123456789"), Ok(()));
}

#[test]
fn test_validate_fractional_values() {
    assert_eq!(Decimal::validate("4.56"), Ok(()));
    assert_eq!(Decimal::validate("-4.56"), Ok(()));
    assert_eq!(Decimal::validate("0.5"), Ok(()));
    assert_eq!(Decimal::validate("0.010"), Ok(()));
}

#[test]
fn test_validate_exponent_values() {
    assert_eq!(Decimal::validate("1e10"), Ok(()));
    assert_eq!(Decimal::validate("1E10"), Ok(()));
    assert_eq!(Decimal::validate("1e+10"), Ok(()));
    assert_eq!(Decimal::validate("1e-10"), Ok(()));
    assert_eq!(Decimal::validate("4.56e-10"), Ok(()));
    assert_eq!(Decimal::validate("-4.56e+10"), Ok(()));
}

#[test]
fn test_validate_digit_limit_boundaries() {
    // Exactly 18 integer digits: ok.
    let int18 = "1".repeat(18);
    assert_eq!(Decimal::validate(&int18), Ok(()));

    // Exactly 17 fraction digits: ok.
    let frac17 = format!("0.{}", "1".repeat(17));
    assert_eq!(Decimal::validate(&frac17), Ok(()));

    // Exactly 9 exponent digits: ok.
    let exp9 = format!("1e{}", "1".repeat(9));
    assert_eq!(Decimal::validate(&exp9), Ok(()));
}

// ---------------------------------------------------------------------
// validate(): negative paths
// ---------------------------------------------------------------------

#[test]
fn test_validate_empty_string() {
    assert_eq!(Decimal::validate(""), Err(DecimalError::Empty));
}

#[test]
fn test_validate_plus_sign_rejected() {
    // Unlike Integer/Integer64, decimal's regex has no `+` alternative at all.
    assert_eq!(Decimal::validate("+42"), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate("+0"), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate("+4.5"), Err(DecimalError::InvalidFormat));
    assert!(Decimal::try_from("+42").is_err());
}

#[test]
fn test_validate_leading_zero() {
    assert_eq!(Decimal::validate("01"), Err(DecimalError::LeadingZero));
    assert_eq!(Decimal::validate("007"), Err(DecimalError::LeadingZero));
    assert_eq!(Decimal::validate("-01"), Err(DecimalError::LeadingZero));
    assert_eq!(Decimal::validate("00"), Err(DecimalError::LeadingZero));
    assert_eq!(Decimal::validate("01.5"), Err(DecimalError::LeadingZero));
}

#[test]
fn test_validate_missing_integer_part() {
    assert_eq!(Decimal::validate(".5"), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate("-.5"), Err(DecimalError::InvalidFormat));
}

#[test]
fn test_validate_missing_fraction_digits() {
    assert_eq!(Decimal::validate("5."), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate("5.e10"), Err(DecimalError::InvalidFormat));
}

#[test]
fn test_validate_missing_exponent_digits() {
    assert_eq!(Decimal::validate("1e"), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate("1e+"), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate("1e-"), Err(DecimalError::InvalidFormat));
}

#[test]
fn test_validate_lone_sign() {
    assert_eq!(Decimal::validate("-"), Err(DecimalError::InvalidFormat));
}

#[test]
fn test_validate_non_digit_characters() {
    assert_eq!(Decimal::validate("abc"), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate("4.5a"), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate("4a.5"), Err(DecimalError::InvalidFormat));
}

#[test]
fn test_validate_internal_and_surrounding_whitespace() {
    assert_eq!(Decimal::validate(" 1"), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate("1 "), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate("1 .5"), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate(" "), Err(DecimalError::InvalidFormat));
}

#[test]
fn test_validate_control_characters() {
    assert_eq!(Decimal::validate("1\0"), Err(DecimalError::InvalidFormat));
    assert_eq!(
        Decimal::validate("4.5\u{0008}"),
        Err(DecimalError::InvalidFormat)
    );
}

#[test]
fn test_validate_mixed_double_signs() {
    assert_eq!(Decimal::validate("--1"), Err(DecimalError::InvalidFormat));
    assert_eq!(Decimal::validate("-+1"), Err(DecimalError::InvalidFormat));
}

#[test]
fn test_validate_unicode_digits_rejected() {
    assert_eq!(
        Decimal::validate("\u{FF11}"),
        Err(DecimalError::InvalidFormat)
    );
}

#[test]
fn test_validate_too_many_integer_digits() {
    let int19 = "1".repeat(19);
    assert_eq!(
        Decimal::validate(&int19),
        Err(DecimalError::TooManyIntegerDigits { count: 19, max: 18 })
    );
}

#[test]
fn test_validate_too_many_fraction_digits() {
    let frac18 = format!("0.{}", "1".repeat(18));
    assert_eq!(
        Decimal::validate(&frac18),
        Err(DecimalError::TooManyFractionDigits { count: 18, max: 17 })
    );
}

#[test]
fn test_validate_too_many_exponent_digits() {
    let exp10 = format!("1e{}", "1".repeat(10));
    assert_eq!(
        Decimal::validate(&exp10),
        Err(DecimalError::TooManyExponentDigits { count: 10, max: 9 })
    );
}

// ---------------------------------------------------------------------
// as_str / as_f64 / into_inner / new_unchecked
// ---------------------------------------------------------------------

#[test]
fn test_as_str_and_into_inner() {
    let d = Decimal::try_from("4.5600").unwrap();
    assert_eq!(d.as_str(), "4.5600");
    assert_eq!(d.into_inner(), "4.5600".to_string());
}

#[test]
fn test_as_f64() {
    assert_eq!(Decimal::try_from("4.56").unwrap().as_f64(), 4.56);
    assert_eq!(Decimal::try_from("0.010").unwrap().as_f64(), 0.01);
    assert_eq!(Decimal::try_from("-0").unwrap().as_f64(), 0.0);
    assert_eq!(Decimal::try_from("1e10").unwrap().as_f64(), 1e10);
}

#[test]
fn test_new_unchecked() {
    let d = Decimal::new_unchecked("4.56");
    assert_eq!(d.as_str(), "4.56");
    assert_eq!(d.as_f64(), 4.56);

    // Bypasses validation entirely — even invalid strings are accepted at construction time.
    let invalid = Decimal::new_unchecked("garbage");
    assert_eq!(invalid.as_str(), "garbage");
}

#[test]
#[should_panic(expected = "Decimal invariant violated")]
fn test_new_unchecked_as_f64_panics_on_invalid_string() {
    Decimal::new_unchecked("garbage").as_f64();
}

// ---------------------------------------------------------------------
// TryFrom / FromStr / Display
// ---------------------------------------------------------------------

#[test]
fn test_try_from_str_valid() {
    let d = Decimal::try_from("4.56").unwrap();
    assert_eq!(d.as_str(), "4.56");
}

#[test]
fn test_try_from_str_invalid() {
    assert!(Decimal::try_from("").is_err());
    assert!(Decimal::try_from("abc").is_err());
    assert!(Decimal::try_from("01").is_err());
}

#[test]
fn test_try_from_string_valid() {
    let s = String::from("4.56");
    let d = Decimal::try_from(s).unwrap();
    assert_eq!(d.as_str(), "4.56");
}

#[test]
fn test_try_from_string_invalid() {
    let s = String::from("+42");
    assert!(Decimal::try_from(s).is_err());
}

#[test]
fn test_from_str_and_display() {
    let d = Decimal::from_str("4.5600").unwrap();
    assert_eq!(format!("{d}"), "4.5600");

    assert!(Decimal::from_str("bad").is_err());
}

#[test]
fn test_error_message_output() {
    let err = Decimal::try_from("+42").unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid value '+42' for type 'decimal': not a valid decimal representation"
    );

    let leading_zero_err = Decimal::try_from("007").unwrap_err();
    assert_eq!(
        leading_zero_err.to_string(),
        "invalid value '007' for type 'decimal': non-zero decimal must not have a leading zero"
    );

    let empty_err = Decimal::try_from("").unwrap_err();
    assert_eq!(
        empty_err.to_string(),
        "invalid value '' for type 'decimal': decimal string must not be empty"
    );
}

#[test]
fn test_decimal_error_display_variants() {
    assert_eq!(
        DecimalError::Empty.to_string(),
        "decimal string must not be empty"
    );
    assert_eq!(
        DecimalError::InvalidFormat.to_string(),
        "not a valid decimal representation"
    );
    assert_eq!(
        DecimalError::LeadingZero.to_string(),
        "non-zero decimal must not have a leading zero"
    );
    assert_eq!(
        DecimalError::TooManyIntegerDigits { count: 19, max: 18 }.to_string(),
        "integer part has 19 digits, exceeding the maximum of 18"
    );
    assert_eq!(
        DecimalError::TooManyFractionDigits { count: 18, max: 17 }.to_string(),
        "fractional part has 18 digits, exceeding the maximum of 17"
    );
    assert_eq!(
        DecimalError::TooManyExponentDigits { count: 10, max: 9 }.to_string(),
        "exponent has 10 digits, exceeding the maximum of 9"
    );
}

// ---------------------------------------------------------------------
// Equality semantics: precision-significant, string-exact
// ---------------------------------------------------------------------

#[test]
fn test_equality_is_precision_significant() {
    let a = Decimal::try_from("0.010").unwrap();
    let b = Decimal::try_from("0.01").unwrap();
    assert_ne!(a, b);
    assert_eq!(a.as_f64(), b.as_f64()); // numerically equal, identity-distinct
}

#[test]
fn test_equality_is_string_exact_not_numeric() {
    // Documented consequence: alternate notations for the same numeric value/precision are
    // also treated as distinct, since equality is string-exact rather than value+precision.
    assert_ne!(
        Decimal::try_from("100").unwrap(),
        Decimal::try_from("100.0").unwrap()
    );
    assert_ne!(
        Decimal::try_from("100").unwrap(),
        Decimal::try_from("1e2").unwrap()
    );
}

#[test]
fn test_equality_reflexive_for_identical_strings() {
    assert_eq!(
        Decimal::try_from("4.5600").unwrap(),
        Decimal::try_from("4.5600").unwrap()
    );
}

#[test]
fn test_hash_consistent_with_eq() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(Decimal::try_from("0.010").unwrap());
    set.insert(Decimal::try_from("0.01").unwrap());
    // Precision-distinct values are distinct set members.
    assert_eq!(set.len(), 2);

    set.insert(Decimal::try_from("0.010").unwrap());
    // Re-inserting an identical string does not grow the set.
    assert_eq!(set.len(), 2);
}

// ---------------------------------------------------------------------
// serde: Serialize — exact integer path
// ---------------------------------------------------------------------

#[test]
fn test_serde_serialize_whole_number_as_json_integer() {
    let d = Decimal::try_from("100").unwrap();
    assert_eq!(serde_json::to_string(&d).unwrap(), "100");

    let neg = Decimal::try_from("-100").unwrap();
    assert_eq!(serde_json::to_string(&neg).unwrap(), "-100");

    let zero = Decimal::try_from("0").unwrap();
    assert_eq!(serde_json::to_string(&zero).unwrap(), "0");
}

// ---------------------------------------------------------------------
// serde: Serialize — lossy f64 path (documented)
// ---------------------------------------------------------------------

#[test]
fn test_serde_serialize_fractional_as_json_number() {
    let d = Decimal::try_from("4.56").unwrap();
    assert_eq!(serde_json::to_string(&d).unwrap(), "4.56");
}

#[test]
fn test_serde_serialize_fractional_drops_trailing_zeros() {
    // Documented lossy behavior: trailing zeros in the fractional part are not preserved
    // through the f64 serialization path.
    let d = Decimal::try_from("4.5600").unwrap();
    assert_eq!(serde_json::to_string(&d).unwrap(), "4.56");
}

// ---------------------------------------------------------------------
// serde: Deserialize — exact paths
// ---------------------------------------------------------------------

#[test]
fn test_serde_deserialize_from_json_integer_exact() {
    let d: Decimal = serde_json::from_str("42").unwrap();
    assert_eq!(d.as_str(), "42");

    let neg: Decimal = serde_json::from_str("-42").unwrap();
    assert_eq!(neg.as_str(), "-42");
}

#[test]
fn test_serde_deserialize_from_json_string_exact() {
    // The precision-preserving path: exact original text, trailing zeros included.
    let d: Decimal = serde_json::from_str("\"4.5600\"").unwrap();
    assert_eq!(d.as_str(), "4.5600");
}

#[test]
fn test_serde_deserialize_from_value_string_exact() {
    let val = serde_json::Value::String("0.010".to_string());
    let d: Decimal = serde_json::from_value(val).unwrap();
    assert_eq!(d.as_str(), "0.010");
}

// ---------------------------------------------------------------------
// serde: Deserialize — lossy f64 path (documented)
// ---------------------------------------------------------------------

#[test]
fn test_serde_deserialize_from_json_fractional_number_reconstructs_value() {
    let d: Decimal = serde_json::from_str("4.56").unwrap();
    assert_eq!(d.as_str(), "4.56");
    assert_eq!(d.as_f64(), 4.56);
}

#[test]
fn test_serde_deserialize_extreme_magnitude_round_trips_without_error() {
    // A spec-valid Decimal ("1e300": 1 integer digit, 3 exponent digits) must not fail to
    // round-trip through its own (de)serializer just because the plain-decimal expansion of
    // its f64 value would exceed the digit limits.
    let original = Decimal::try_from("1e300").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let round_tripped: Decimal = serde_json::from_str(&json).unwrap();
    assert_eq!(round_tripped.as_f64(), original.as_f64());
    // Reconstructed via scientific notation, not a 301-digit plain expansion.
    assert!(Decimal::validate(round_tripped.as_str()).is_ok());
}

#[test]
fn test_serde_deserialize_extreme_negative_magnitude() {
    let original = Decimal::try_from("-1e300").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let round_tripped: Decimal = serde_json::from_str(&json).unwrap();
    assert_eq!(round_tripped.as_f64(), original.as_f64());
}

#[test]
fn test_serde_deserialize_extreme_small_magnitude() {
    let original = Decimal::try_from("1e-300").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let round_tripped: Decimal = serde_json::from_str(&json).unwrap();
    assert_eq!(round_tripped.as_f64(), original.as_f64());
}

// ---------------------------------------------------------------------
// serde: round-trip equality — only where it's actually guaranteed
// ---------------------------------------------------------------------

#[test]
fn test_serde_roundtrip_whole_number_is_exact() {
    let original = Decimal::try_from("123456789012345678").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let round_tripped: Decimal = serde_json::from_str(&json).unwrap();
    assert_eq!(original, round_tripped);
}

#[test]
fn test_serde_roundtrip_via_json_string_is_exact() {
    let original = Decimal::try_from("0.010").unwrap();
    let json = serde_json::to_string(original.as_str()).unwrap();
    let round_tripped: Decimal = serde_json::from_str(&json).unwrap();
    assert_eq!(original, round_tripped);
}

#[test]
fn test_serde_roundtrip_fractional_json_number_is_lossy_by_design() {
    let original = Decimal::try_from("0.010").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    assert_eq!(json, "0.01"); // trailing zero already dropped on serialize
    let round_tripped: Decimal = serde_json::from_str(&json).unwrap();
    assert_ne!(original, round_tripped); // documented: not byte-identical
    assert_eq!(original.as_f64(), round_tripped.as_f64()); // numerically equal
}

// ---------------------------------------------------------------------
// serde: negative paths
// ---------------------------------------------------------------------

#[test]
fn test_serde_deserialize_nan_and_infinite_rejected() {
    // Real JSON text can never encode NaN/Infinity, so serde_json would reject these as a
    // syntax error before our Deserialize ever runs — that wouldn't exercise our own
    // `is_finite` check. Drive `visit_f64` directly (as e.g. a binary format like CBOR could)
    // to prove the check itself works.
    use serde::Deserialize;
    use serde::de::IntoDeserializer;
    use serde::de::value::Error as ValueError;

    let nan_result: Result<Decimal, ValueError> =
        Decimal::deserialize(f64::NAN.into_deserializer());
    assert!(nan_result.is_err());

    let inf_result: Result<Decimal, ValueError> =
        Decimal::deserialize(f64::INFINITY.into_deserializer());
    assert!(inf_result.is_err());

    let neg_inf_result: Result<Decimal, ValueError> =
        Decimal::deserialize(f64::NEG_INFINITY.into_deserializer());
    assert!(neg_inf_result.is_err());
}

#[test]
fn test_serde_deserialize_invalid_string_value() {
    assert!(serde_json::from_str::<Decimal>("\"+42\"").is_err());
    assert!(serde_json::from_str::<Decimal>("\"\"").is_err());
    assert!(serde_json::from_str::<Decimal>("\"abc\"").is_err());
}

#[test]
fn test_serde_deserialize_invalid_type() {
    assert!(serde_json::from_str::<Decimal>("true").is_err());
    assert!(serde_json::from_str::<Decimal>("null").is_err());
    assert!(serde_json::from_str::<Decimal>("[]").is_err());
    assert!(serde_json::from_str::<Decimal>("{}").is_err());
}

#[test]
fn test_serde_deserialize_malformed_json() {
    assert!(serde_json::from_str::<Decimal>("").is_err());
    assert!(serde_json::from_str::<Decimal>("abc").is_err());
}
