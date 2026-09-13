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

fn cnt3_violated(result: &FhirCoreResult<Count>) -> bool {
    matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "cnt-3", .. }
        ))
    )
}

// --- Happy path ---

#[test]
fn test_new_with_whole_value_and_code_succeeds() {
    let count = Count::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("1")),
        None,
        Vec::new(),
    );
    assert!(count.is_ok());
    assert_eq!(count.unwrap().value(), Some(&dec("5")));
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
    let count = Count::new(None, None, None, None, None, None, vec![ext]);
    assert!(count.is_ok());
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let count = Count::new_unchecked(Some(dec("5.5")), None, None, None, None, None, Vec::new());
    assert_eq!(count.value(), Some(&dec("5.5")));
}

// --- Negative / boundary ---

#[test]
fn test_new_with_nothing_fails_ele1() {
    let result = Count::new(None, None, None, None, None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ele-1", .. }
        ))
    ));
}

#[test]
fn test_new_with_code_without_system_fails_qty3() {
    let result = Count::new(None, None, None, None, Some(cd("1")), None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "qty-3", .. }
        ))
    ));
}

#[test]
fn test_new_with_value_without_code_fails_cnt3() {
    let result = Count::new(Some(dec("5")), None, None, None, None, None, Vec::new());
    assert!(cnt3_violated(&result));
}

#[test]
fn test_new_with_non_one_code_fails_cnt3() {
    let result = Count::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("mg")),
        None,
        Vec::new(),
    );
    assert!(cnt3_violated(&result));
}

#[test]
fn test_new_with_non_ucum_system_fails_cnt3() {
    let result = Count::new(
        None,
        None,
        None,
        Some(uri("http://example.org/other")),
        Some(cd("1")),
        None,
        Vec::new(),
    );
    assert!(cnt3_violated(&result));
}

#[test]
fn test_new_with_fractional_value_fails_cnt3() {
    let result = Count::new(
        Some(dec("5.5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("1")),
        None,
        Vec::new(),
    );
    assert!(cnt3_violated(&result));
}

#[test]
fn test_new_unchecked_bypasses_cnt3() {
    let count = Count::new_unchecked(Some(dec("5.5")), None, None, None, None, None, Vec::new());
    assert_eq!(count.value(), Some(&dec("5.5")));
}

// --- Serde ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_all_fields() {
    let count = Count::new(
        Some(dec("5")),
        Some(cd(">")),
        None,
        Some(uri(UCUM)),
        Some(cd("1")),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&count).unwrap();
    let back: Count = serde_json::from_str(&json).unwrap();
    assert_eq!(count, back);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_empty_object_fails_ele1() {
    let result: Result<Count, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_fractional_value_fails_cnt3() {
    let result: Result<Count, _> = serde_json::from_str(
        r#"{"value": 5.5, "system": "http://unitsofmeasure.org", "code": "1"}"#,
    );
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<Count, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}
