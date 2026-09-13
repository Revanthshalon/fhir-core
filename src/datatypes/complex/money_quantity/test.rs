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

fn mtqy1_violated(result: &FhirCoreResult<MoneyQuantity>) -> bool {
    matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "mtqy-1", .. }
        ))
    )
}

// --- Happy path ---

#[test]
fn test_new_with_value_and_code_succeeds() {
    let money = MoneyQuantity::new(
        Some(dec("10.50")),
        None,
        None,
        Some(uri(ISO_4217)),
        Some(cd("USD")),
        None,
        Vec::new(),
    );
    assert!(money.is_ok());
    assert_eq!(money.unwrap().value(), Some(&dec("10.50")));
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
    let money = MoneyQuantity::new(None, None, None, None, None, None, vec![ext]);
    assert!(money.is_ok());
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let money =
        MoneyQuantity::new_unchecked(Some(dec("10")), None, None, None, None, None, Vec::new());
    assert_eq!(money.value(), Some(&dec("10")));
}

// --- Negative / boundary ---

#[test]
fn test_new_with_nothing_fails_ele1() {
    let result = MoneyQuantity::new(None, None, None, None, None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ele-1", .. }
        ))
    ));
}

#[test]
fn test_new_with_code_without_system_fails_qty3() {
    let result = MoneyQuantity::new(None, None, None, None, Some(cd("USD")), None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "qty-3", .. }
        ))
    ));
}

#[test]
fn test_new_with_value_without_code_fails_mtqy1() {
    let result = MoneyQuantity::new(Some(dec("10")), None, None, None, None, None, Vec::new());
    assert!(mtqy1_violated(&result));
}

#[test]
fn test_new_with_non_iso4217_system_fails_mtqy1() {
    let result = MoneyQuantity::new(
        None,
        None,
        None,
        Some(uri("http://example.org/other")),
        Some(cd("USD")),
        None,
        Vec::new(),
    );
    assert!(mtqy1_violated(&result));
}

#[test]
fn test_new_unchecked_bypasses_mtqy1() {
    let money =
        MoneyQuantity::new_unchecked(Some(dec("10")), None, None, None, None, None, Vec::new());
    assert_eq!(money.value(), Some(&dec("10")));
}

// --- Serde ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_all_fields() {
    let money = MoneyQuantity::new(
        Some(dec("10.5")),
        Some(cd(">")),
        None,
        Some(uri(ISO_4217)),
        Some(cd("USD")),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&money).unwrap();
    let back: MoneyQuantity = serde_json::from_str(&json).unwrap();
    assert_eq!(money, back);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_empty_object_fails_ele1() {
    let result: Result<MoneyQuantity, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_value_without_code_fails_mtqy1() {
    let result: Result<MoneyQuantity, _> = serde_json::from_str(r#"{"value": 10.50}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<MoneyQuantity, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}
