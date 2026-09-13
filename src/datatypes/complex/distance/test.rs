use super::*;
use crate::errors::FhirCoreError;
use crate::types::Boolean;

fn dec(s: &str) -> Primitive<Decimal> {
    Primitive::from_value(Decimal::try_from(s).unwrap())
}

fn cd(s: &str) -> Primitive<Code> {
    Primitive::from_value(Code::new(s).unwrap())
}

fn uri(s: &str) -> Primitive<Uri> {
    Primitive::from_value(Uri::new(s).unwrap())
}

fn dis1_violated(result: &FhirCoreResult<Distance>) -> bool {
    matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "dis-1", .. }
        ))
    )
}

// --- Happy path ---

#[test]
fn test_new_with_value_and_code_succeeds() {
    let distance = Distance::new(
        Some(dec("5.4")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("m")),
        None,
        Vec::new(),
    );
    assert!(distance.is_ok());
    assert_eq!(distance.unwrap().value(), Some(&dec("5.4")));
}

#[test]
fn test_new_with_negative_value_succeeds() {
    // Unlike Age, Distance has no positivity requirement.
    let distance = Distance::new(
        Some(dec("-5.4")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("m")),
        None,
        Vec::new(),
    );
    assert!(distance.is_ok());
}

#[test]
fn test_new_with_extension_only_succeeds() {
    let ext = Extension::new(
        uri("http://example.org/fhir/x"),
        None,
        Vec::new(),
        Some(crate::datatypes::complex::ExtensionValue::Boolean(
            Primitive::from_value(Boolean::new(true)),
        )),
    )
    .unwrap();
    let distance = Distance::new(None, None, None, None, None, None, vec![ext]);
    assert!(distance.is_ok());
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let distance =
        Distance::new_unchecked(Some(dec("5.4")), None, None, None, None, None, Vec::new());
    assert_eq!(distance.value(), Some(&dec("5.4")));
}

// --- Negative / boundary ---

#[test]
fn test_new_with_nothing_fails_ele1() {
    let result = Distance::new(None, None, None, None, None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ele-1", .. }
        ))
    ));
}

#[test]
fn test_new_with_code_without_system_fails_qty3() {
    let result = Distance::new(None, None, None, None, Some(cd("m")), None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "qty-3", .. }
        ))
    ));
}

#[test]
fn test_new_with_value_without_code_fails_dis1() {
    let result = Distance::new(Some(dec("5.4")), None, None, None, None, None, Vec::new());
    assert!(dis1_violated(&result));
}

#[test]
fn test_new_with_non_ucum_system_fails_dis1() {
    let result = Distance::new(
        None,
        None,
        None,
        Some(uri("http://example.org/other")),
        Some(cd("m")),
        None,
        Vec::new(),
    );
    assert!(dis1_violated(&result));
}

#[test]
fn test_new_unchecked_bypasses_dis1() {
    let distance =
        Distance::new_unchecked(Some(dec("5.4")), None, None, None, None, None, Vec::new());
    assert_eq!(distance.value(), Some(&dec("5.4")));
}

// --- Serde ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_all_fields() {
    let distance = Distance::new(
        Some(dec("5.4")),
        Some(cd(">")),
        None,
        Some(uri(UCUM)),
        Some(cd("m")),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&distance).unwrap();
    let back: Distance = serde_json::from_str(&json).unwrap();
    assert_eq!(distance, back);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_empty_object_fails_ele1() {
    let result: Result<Distance, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_value_without_code_fails_dis1() {
    let result: Result<Distance, _> = serde_json::from_str(r#"{"value": 5.4}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<Distance, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}
