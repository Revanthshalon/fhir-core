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

// --- Happy path ---

#[test]
fn test_new_with_value_only_succeeds() {
    let quantity = SimpleQuantity::new(Some(dec("5.4")), None, None, None, None, Vec::new());
    assert!(quantity.is_ok());
    assert_eq!(quantity.unwrap().value(), Some(&dec("5.4")));
}

#[test]
fn test_new_with_all_fields_succeeds() {
    let quantity = SimpleQuantity::new(
        Some(dec("5.4")),
        None,
        Some(uri("http://unitsofmeasure.org")),
        Some(cd("mg")),
        Some(FhirString::new("q1").unwrap()),
        Vec::new(),
    );
    assert!(quantity.is_ok());
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
    let quantity = SimpleQuantity::new(None, None, None, None, None, vec![ext]);
    assert!(quantity.is_ok());
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let quantity = SimpleQuantity::new_unchecked(None, None, None, None, None, Vec::new());
    assert!(quantity.value().is_none());
}

// --- Negative / boundary ---

#[test]
fn test_new_with_nothing_fails_ele1() {
    let result = SimpleQuantity::new(None, None, None, None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ele-1", .. }
        ))
    ));
}

#[test]
fn test_new_with_code_without_system_fails_qty3() {
    let result = SimpleQuantity::new(None, None, None, Some(cd("mg")), None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "qty-3", .. }
        ))
    ));
}

#[test]
fn test_new_unchecked_bypasses_qty3() {
    let quantity =
        SimpleQuantity::new_unchecked(None, None, None, Some(cd("mg")), None, Vec::new());
    assert_eq!(quantity.code(), Some(&cd("mg")));
    assert!(quantity.system().is_none());
}

// --- Ordering ---

#[test]
fn test_partial_ord_same_unit_orders_by_value() {
    let low = SimpleQuantity::new(Some(dec("3")), None, None, None, None, Vec::new()).unwrap();
    let high = SimpleQuantity::new(Some(dec("5")), None, None, None, None, Vec::new()).unwrap();
    assert!(low < high);
}

#[test]
fn test_partial_ord_different_units_incomparable() {
    let mg = SimpleQuantity::new(
        Some(dec("5")),
        None,
        Some(uri("http://unitsofmeasure.org")),
        Some(cd("mg")),
        None,
        Vec::new(),
    )
    .unwrap();
    let kg = SimpleQuantity::new(
        Some(dec("5")),
        None,
        Some(uri("http://unitsofmeasure.org")),
        Some(cd("kg")),
        None,
        Vec::new(),
    )
    .unwrap();
    assert_eq!(mg.partial_cmp(&kg), None);
}

// --- Serde ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_all_fields() {
    let quantity = SimpleQuantity::new(
        Some(dec("5.4")),
        None,
        Some(uri("http://unitsofmeasure.org")),
        Some(cd("mg")),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&quantity).unwrap();
    let back: SimpleQuantity = serde_json::from_str(&json).unwrap();
    assert_eq!(quantity, back);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_empty_object_fails_ele1() {
    let result: Result<SimpleQuantity, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_ignores_comparator_field() {
    // sqty-1: comparator is structurally absent; a payload carrying one is ignored
    // like any other unknown field, not rejected.
    let quantity: SimpleQuantity =
        serde_json::from_str(r#"{"value": 5.4, "comparator": "<"}"#).unwrap();
    assert_eq!(quantity.value(), Some(&dec("5.4")));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialize_never_emits_comparator() {
    let quantity =
        SimpleQuantity::new(Some(dec("5.4")), None, None, None, None, Vec::new()).unwrap();
    let json = serde_json::to_value(&quantity).unwrap();
    assert!(json.get("comparator").is_none());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<SimpleQuantity, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}
