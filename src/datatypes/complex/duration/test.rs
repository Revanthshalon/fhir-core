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

fn text(s: &str) -> Primitive<FhirString> {
    Primitive::from_value(FhirString::new(s).unwrap())
}

fn drt1_violated(result: &FhirCoreResult<Duration>) -> bool {
    matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "drt-1", .. }
        ))
    )
}

// --- Happy path ---

#[test]
fn test_new_with_value_code_and_ucum_system_succeeds() {
    let duration = Duration::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("d")),
        None,
        Vec::new(),
    );
    assert!(duration.is_ok());
    assert_eq!(duration.unwrap().value(), Some(&dec("5")));
}

#[test]
fn test_new_with_all_fields_succeeds() {
    let duration = Duration::new(
        Some(dec("5")),
        Some(cd(">")),
        Some(text("days")),
        Some(uri(UCUM)),
        Some(cd("d")),
        Some(FhirString::new("du1").unwrap()),
        Vec::new(),
    );
    assert!(duration.is_ok());
    let duration = duration.unwrap();
    assert_eq!(duration.value(), Some(&dec("5")));
    assert_eq!(duration.comparator(), Some(&cd(">")));
    assert_eq!(duration.unit(), Some(&text("days")));
    assert_eq!(duration.system(), Some(&uri(UCUM)));
    assert_eq!(duration.code(), Some(&cd("d")));
    assert_eq!(duration.id().unwrap().as_str(), "du1");
}

#[test]
fn test_new_with_value_only_no_code_succeeds() {
    // drt-1 only constrains when code is present.
    let duration = Duration::new(Some(dec("5")), None, None, None, None, None, Vec::new());
    assert!(duration.is_ok());
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
    let duration = Duration::new(None, None, None, None, None, None, vec![ext]);
    assert!(duration.is_ok());
    assert_eq!(duration.unwrap().extensions().len(), 1);
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let duration = Duration::new_unchecked(None, None, None, None, Some(cd("d")), None, Vec::new());
    assert_eq!(duration.code(), Some(&cd("d")));
}

// --- Negative / boundary ---

#[test]
fn test_new_with_nothing_fails_ele1() {
    let result = Duration::new(None, None, None, None, None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ele-1", .. }
        ))
    ));
}

#[test]
fn test_new_with_code_without_system_fails_qty3() {
    let result = Duration::new(None, None, None, None, Some(cd("d")), None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "qty-3", .. }
        ))
    ));
}

#[test]
fn test_new_with_code_and_non_ucum_system_fails_drt1() {
    let result = Duration::new(
        Some(dec("5")),
        None,
        None,
        Some(uri("http://example.org/other")),
        Some(cd("d")),
        None,
        Vec::new(),
    );
    assert!(drt1_violated(&result));
}

#[test]
fn test_new_with_code_and_ucum_system_but_no_value_fails_drt1() {
    let result = Duration::new(
        None,
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("d")),
        None,
        Vec::new(),
    );
    assert!(drt1_violated(&result));
}

#[test]
fn test_new_unchecked_bypasses_drt1() {
    let duration = Duration::new_unchecked(
        None,
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("d")),
        None,
        Vec::new(),
    );
    assert_eq!(duration.code(), Some(&cd("d")));
    assert!(duration.value().is_none());
}

// --- Ordering ---

#[test]
fn test_partial_ord_same_unit_orders_by_value() {
    let short = Duration::new(
        Some(dec("3")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("d")),
        None,
        Vec::new(),
    )
    .unwrap();
    let long = Duration::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("d")),
        None,
        Vec::new(),
    )
    .unwrap();
    assert!(short < long);
}

#[test]
fn test_partial_ord_different_units_incomparable() {
    let days = Duration::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("d")),
        None,
        Vec::new(),
    )
    .unwrap();
    let hours = Duration::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("h")),
        None,
        Vec::new(),
    )
    .unwrap();
    assert_eq!(days.partial_cmp(&hours), None);
}

#[test]
fn test_partial_ord_identical_instance_is_equal() {
    let duration = Duration::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("d")),
        None,
        Vec::new(),
    )
    .unwrap();
    assert_eq!(
        duration.partial_cmp(&duration),
        Some(std::cmp::Ordering::Equal)
    );
}

#[test]
fn test_partial_ord_equal_magnitude_different_metadata_is_incomparable() {
    let a = Duration::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("d")),
        Some(FhirString::new("a").unwrap()),
        Vec::new(),
    )
    .unwrap();
    let b = Duration::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("d")),
        Some(FhirString::new("b").unwrap()),
        Vec::new(),
    )
    .unwrap();
    assert_ne!(a, b);
    assert_eq!(a.partial_cmp(&b), None);
}

// --- Serde ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_all_fields() {
    let duration = Duration::new(
        Some(dec("5")),
        Some(cd(">")),
        Some(text("days")),
        Some(uri(UCUM)),
        Some(cd("d")),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&duration).unwrap();
    let back: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(duration, back);
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
    let duration = Duration::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("d")),
        Some(FhirString::new("du1").unwrap()),
        vec![ext],
    )
    .unwrap();
    let json = serde_json::to_string(&duration).unwrap();
    let back: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(duration, back);
    assert_eq!(back.id().unwrap().as_str(), "du1");
    assert_eq!(back.extensions().len(), 1);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_unknown_field_ignored() {
    let duration: Duration = serde_json::from_str(
        r#"{"value": 5, "system": "http://unitsofmeasure.org", "code": "d", "unknown": 1}"#,
    )
    .unwrap();
    assert_eq!(duration.value(), Some(&dec("5")));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_empty_object_fails_ele1() {
    let result: Result<Duration, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_code_without_value_fails_drt1() {
    let result: Result<Duration, _> =
        serde_json::from_str(r#"{"system": "http://unitsofmeasure.org", "code": "d"}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<Duration, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}
