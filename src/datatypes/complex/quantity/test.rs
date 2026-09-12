use super::*;
use crate::errors::FhirCoreError;
use crate::types::Boolean;

fn dec(s: &str) -> Primitive<Decimal> {
    Primitive::from_value(Decimal::try_from(s).unwrap())
}

fn cd(s: &str) -> Primitive<Code> {
    Primitive::from_value(Code::new(s).unwrap())
}

fn text(s: &str) -> Primitive<FhirString> {
    Primitive::from_value(FhirString::new(s).unwrap())
}

fn uri(s: &str) -> Primitive<Uri> {
    Primitive::from_value(Uri::new(s).unwrap())
}

// --- Happy path ---

#[test]
fn test_new_with_value_only_succeeds() {
    let quantity = Quantity::new(Some(dec("5.4")), None, None, None, None, None, Vec::new());
    assert!(quantity.is_ok());
    let quantity = quantity.unwrap();
    assert_eq!(quantity.value(), Some(&dec("5.4")));
    assert!(quantity.code().is_none());
}

#[test]
fn test_new_with_code_and_system_succeeds() {
    let quantity = Quantity::new(
        None,
        None,
        None,
        Some(uri("http://unitsofmeasure.org")),
        Some(cd("mg")),
        None,
        Vec::new(),
    );
    assert!(quantity.is_ok());
}

#[test]
fn test_new_with_all_fields_succeeds() {
    let quantity = Quantity::new(
        Some(dec("5.4")),
        Some(cd("<")),
        Some(text("mg")),
        Some(uri("http://unitsofmeasure.org")),
        Some(cd("mg")),
        Some(FhirString::new("q1").unwrap()),
        Vec::new(),
    );
    assert!(quantity.is_ok());
    let quantity = quantity.unwrap();
    assert_eq!(quantity.value(), Some(&dec("5.4")));
    assert_eq!(quantity.comparator(), Some(&cd("<")));
    assert_eq!(quantity.unit(), Some(&text("mg")));
    assert_eq!(quantity.system(), Some(&uri("http://unitsofmeasure.org")));
    assert_eq!(quantity.code(), Some(&cd("mg")));
    assert_eq!(quantity.id().unwrap().as_str(), "q1");
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
    let quantity = Quantity::new(None, None, None, None, None, None, vec![ext]);
    assert!(quantity.is_ok());
    assert_eq!(quantity.unwrap().extensions().len(), 1);
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let quantity = Quantity::new_unchecked(None, None, None, None, None, None, Vec::new());
    assert!(quantity.value().is_none());
}

// --- Negative / boundary ---

#[test]
fn test_new_with_nothing_fails_ele1() {
    let result = Quantity::new(None, None, None, None, None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ele-1", .. }
        ))
    ));
}

#[test]
fn test_new_with_code_without_system_fails_qty3() {
    let result = Quantity::new(None, None, None, None, Some(cd("mg")), None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "qty-3", .. }
        ))
    ));
}

#[test]
fn test_new_with_system_without_code_succeeds() {
    // qty-3 only constrains code -> system, not the reverse.
    let quantity = Quantity::new(
        None,
        None,
        None,
        Some(uri("http://unitsofmeasure.org")),
        None,
        None,
        Vec::new(),
    );
    assert!(quantity.is_ok());
}

#[test]
fn test_new_unchecked_bypasses_qty3() {
    let quantity =
        Quantity::new_unchecked(None, None, None, None, Some(cd("mg")), None, Vec::new());
    assert_eq!(quantity.code(), Some(&cd("mg")));
    assert!(quantity.system().is_none());
}

// --- Serde: happy path ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_all_fields() {
    let quantity = Quantity::new(
        Some(dec("5.4")),
        Some(cd("<")),
        Some(text("mg")),
        Some(uri("http://unitsofmeasure.org")),
        Some(cd("mg")),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&quantity).unwrap();
    let back: Quantity = serde_json::from_str(&json).unwrap();
    assert_eq!(quantity, back);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_with_id_and_extension() {
    let ext = Extension::new(
        uri("http://example.org/fhir/x"),
        None,
        Vec::new(),
        Some(crate::datatypes::complex::ExtensionValue::Boolean(
            Primitive::from_value(Boolean::new(true)),
        )),
    )
    .unwrap();
    let quantity = Quantity::new(
        Some(dec("5.4")),
        None,
        None,
        None,
        None,
        Some(FhirString::new("q1").unwrap()),
        vec![ext],
    )
    .unwrap();
    let json = serde_json::to_string(&quantity).unwrap();
    let back: Quantity = serde_json::from_str(&json).unwrap();
    assert_eq!(quantity, back);
    assert_eq!(back.id().unwrap().as_str(), "q1");
    assert_eq!(back.extensions().len(), 1);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialize_only_populated_fields() {
    let quantity =
        Quantity::new(Some(dec("5.4")), None, None, None, None, None, Vec::new()).unwrap();
    let json = serde_json::to_value(&quantity).unwrap();
    assert_eq!(json, serde_json::json!({"value": 5.4}));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_unknown_field_ignored() {
    let quantity: Quantity = serde_json::from_str(r#"{"value": 5.4, "unknown": 1}"#).unwrap();
    assert_eq!(quantity.value(), Some(&dec("5.4")));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_value_with_companion() {
    let quantity: Quantity =
        serde_json::from_str(r#"{"value": 5.4, "_value": {"id": "vid"}}"#).unwrap();
    assert_eq!(quantity.value().unwrap().id().unwrap().as_str(), "vid");
}

// --- Serde: negative / boundary ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_empty_object_fails_ele1() {
    let result: Result<Quantity, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_code_without_system_fails_qty3() {
    let result: Result<Quantity, _> = serde_json::from_str(r#"{"code": "mg"}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_invalid_decimal_value_fails() {
    let result: Result<Quantity, _> = serde_json::from_str(r#"{"value": "not-a-number"}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_invalid_code_fails() {
    let result: Result<Quantity, _> =
        serde_json::from_str(r#"{"system": "http://unitsofmeasure.org", "code": " bad "}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<Quantity, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}
