use super::*;
use crate::errors::FhirCoreError;

fn dt(s: &str) -> Primitive<DateTime> {
    Primitive::from_value(DateTime::new(s).unwrap())
}

#[test]
fn test_new_with_start_only_succeeds() {
    let period = Period::new(Some(dt("2024-01-01T00:00:00Z")), None, None, Vec::new());
    assert!(period.is_ok());
    let period = period.unwrap();
    assert_eq!(period.start(), Some(&dt("2024-01-01T00:00:00Z")));
    assert!(period.end().is_none());
}

#[test]
fn test_new_with_end_only_succeeds() {
    let period = Period::new(None, Some(dt("2024-12-31T23:59:59Z")), None, Vec::new());
    assert!(period.is_ok());
}

#[test]
fn test_new_with_both_succeeds() {
    let period = Period::new(
        Some(dt("2024-01-01T00:00:00Z")),
        Some(dt("2024-12-31T23:59:59Z")),
        None,
        Vec::new(),
    );
    assert!(period.is_ok());
}

#[test]
fn test_new_with_extension_only_succeeds() {
    use crate::datatypes::complex::ExtensionValue;
    use crate::types::{Boolean, Uri};

    let ext = Extension::new(
        Primitive::from_value(Uri::new("http://example.org/fhir/x").unwrap()),
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(true),
        ))),
    )
    .unwrap();
    let period = Period::new(None, None, None, vec![ext]);
    assert!(period.is_ok());
}

#[test]
fn test_new_with_nothing_fails_ele1() {
    let result = Period::new(None, None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ele-1", .. }
        ))
    ));
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let period = Period::new_unchecked(None, None, None, Vec::new());
    assert!(period.start().is_none());
    assert!(period.end().is_none());
}

#[test]
fn test_id_accessor() {
    let period = Period::new(
        Some(dt("2024-01-01T00:00:00Z")),
        None,
        Some(FhirString::new("p1").unwrap()),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(period.id().unwrap().as_str(), "p1");
}

#[test]
fn test_extensions_accessor() {
    use crate::datatypes::complex::ExtensionValue;
    use crate::types::{Boolean, Uri};

    let ext = Extension::new(
        Primitive::from_value(Uri::new("http://example.org/fhir/x").unwrap()),
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(true),
        ))),
    )
    .unwrap();
    let period = Period::new(None, None, None, vec![ext]).unwrap();
    assert_eq!(period.extensions().len(), 1);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_start_and_end() {
    let period = Period::new(
        Some(dt("2024-01-01T00:00:00Z")),
        Some(dt("2024-12-31T23:59:59Z")),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&period).unwrap();
    let back: Period = serde_json::from_str(&json).unwrap();
    assert_eq!(period, back);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialize_only_start() {
    let period = Period::new(Some(dt("2024-01-01T00:00:00Z")), None, None, Vec::new()).unwrap();
    let json = serde_json::to_value(&period).unwrap();
    assert_eq!(json, serde_json::json!({"start": "2024-01-01T00:00:00Z"}));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_missing_everything_fails_ele1() {
    let result: Result<Period, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_unknown_field_ignored() {
    let period: Period =
        serde_json::from_str(r#"{"start": "2024-01-01T00:00:00Z", "unknown": 1}"#).unwrap();
    assert_eq!(period.start(), Some(&dt("2024-01-01T00:00:00Z")));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<Period, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_with_id_and_extension() {
    use crate::datatypes::complex::ExtensionValue;
    use crate::types::{Boolean, Uri};

    let ext = Extension::new(
        Primitive::from_value(Uri::new("http://example.org/fhir/x").unwrap()),
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(true),
        ))),
    )
    .unwrap();
    let period = Period::new(None, None, Some(FhirString::new("p1").unwrap()), vec![ext]).unwrap();
    let json = serde_json::to_string(&period).unwrap();
    let back: Period = serde_json::from_str(&json).unwrap();
    assert_eq!(period, back);
    assert_eq!(back.id().unwrap().as_str(), "p1");
    assert_eq!(back.extensions().len(), 1);
}
