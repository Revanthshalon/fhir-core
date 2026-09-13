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

fn age1_violated(result: &FhirCoreResult<Age>) -> bool {
    matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "age-1", .. }
        ))
    )
}

// --- Happy path ---

#[test]
fn test_new_with_positive_value_and_code_succeeds() {
    let age = Age::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("a")),
        None,
        Vec::new(),
    );
    assert!(age.is_ok());
    let age = age.unwrap();
    assert_eq!(age.value(), Some(&dec("5")));
    assert_eq!(age.code(), Some(&cd("a")));
}

#[test]
fn test_new_with_all_fields_succeeds() {
    let age = Age::new(
        Some(dec("5")),
        Some(cd(">")),
        Some(text("years")),
        Some(uri(UCUM)),
        Some(cd("a")),
        Some(FhirString::new("a1").unwrap()),
        Vec::new(),
    );
    assert!(age.is_ok());
    let age = age.unwrap();
    assert_eq!(age.value(), Some(&dec("5")));
    assert_eq!(age.comparator(), Some(&cd(">")));
    assert_eq!(age.unit(), Some(&text("years")));
    assert_eq!(age.system(), Some(&uri(UCUM)));
    assert_eq!(age.code(), Some(&cd("a")));
    assert_eq!(age.id().unwrap().as_str(), "a1");
}

#[test]
fn test_new_with_code_and_system_no_value_succeeds() {
    let age = Age::new(
        None,
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("a")),
        None,
        Vec::new(),
    );
    assert!(age.is_ok());
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
    let age = Age::new(None, None, None, None, None, None, vec![ext]);
    assert!(age.is_ok());
    assert_eq!(age.unwrap().extensions().len(), 1);
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let age = Age::new_unchecked(Some(dec("-5")), None, None, None, None, None, Vec::new());
    assert_eq!(age.value(), Some(&dec("-5")));
}

// --- Negative / boundary ---

#[test]
fn test_new_with_nothing_fails_ele1() {
    let result = Age::new(None, None, None, None, None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ele-1", .. }
        ))
    ));
}

#[test]
fn test_new_with_code_without_system_fails_qty3() {
    let result = Age::new(None, None, None, None, Some(cd("a")), None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "qty-3", .. }
        ))
    ));
}

#[test]
fn test_new_with_value_without_code_fails_age1() {
    let result = Age::new(Some(dec("5")), None, None, None, None, None, Vec::new());
    assert!(age1_violated(&result));
}

#[test]
fn test_new_with_zero_value_fails_age1() {
    let result = Age::new(
        Some(dec("0")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("a")),
        None,
        Vec::new(),
    );
    assert!(age1_violated(&result));
}

#[test]
fn test_new_with_negative_value_fails_age1() {
    let result = Age::new(
        Some(dec("-1")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("a")),
        None,
        Vec::new(),
    );
    assert!(age1_violated(&result));
}

#[test]
fn test_new_with_non_ucum_system_fails_age1() {
    let result = Age::new(
        None,
        None,
        None,
        Some(uri("http://example.org/other")),
        Some(cd("a")),
        None,
        Vec::new(),
    );
    assert!(age1_violated(&result));
}

#[test]
fn test_new_unchecked_bypasses_age1() {
    let age = Age::new_unchecked(Some(dec("-1")), None, None, None, None, None, Vec::new());
    assert_eq!(age.value(), Some(&dec("-1")));
}

// --- Ordering ---

#[test]
fn test_partial_ord_same_unit_orders_by_value() {
    let young = Age::new(
        Some(dec("3")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("a")),
        None,
        Vec::new(),
    )
    .unwrap();
    let old = Age::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("a")),
        None,
        Vec::new(),
    )
    .unwrap();
    assert!(young < old);
}

#[test]
fn test_partial_ord_different_units_incomparable() {
    let years = Age::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("a")),
        None,
        Vec::new(),
    )
    .unwrap();
    let months = Age::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("mo")),
        None,
        Vec::new(),
    )
    .unwrap();
    assert_eq!(years.partial_cmp(&months), None);
}

#[test]
fn test_partial_ord_identical_instance_is_equal() {
    let age = Age::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("a")),
        None,
        Vec::new(),
    )
    .unwrap();
    assert_eq!(age.partial_cmp(&age), Some(std::cmp::Ordering::Equal));
}

#[test]
fn test_partial_ord_equal_magnitude_different_metadata_is_incomparable() {
    // Same value/comparator/unit, but differing `id` makes PartialEq false —
    // partial_cmp must not report Some(Equal) when PartialEq disagrees.
    let a = Age::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("a")),
        Some(FhirString::new("a").unwrap()),
        Vec::new(),
    )
    .unwrap();
    let b = Age::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("a")),
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
    let age = Age::new(
        Some(dec("5")),
        Some(cd(">")),
        Some(text("years")),
        Some(uri(UCUM)),
        Some(cd("a")),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&age).unwrap();
    let back: Age = serde_json::from_str(&json).unwrap();
    assert_eq!(age, back);
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
    let age = Age::new(
        Some(dec("5")),
        None,
        None,
        Some(uri(UCUM)),
        Some(cd("a")),
        Some(FhirString::new("a1").unwrap()),
        vec![ext],
    )
    .unwrap();
    let json = serde_json::to_string(&age).unwrap();
    let back: Age = serde_json::from_str(&json).unwrap();
    assert_eq!(age, back);
    assert_eq!(back.id().unwrap().as_str(), "a1");
    assert_eq!(back.extensions().len(), 1);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_unknown_field_ignored() {
    let age: Age = serde_json::from_str(
        r#"{"value": 5, "system": "http://unitsofmeasure.org", "code": "a", "unknown": 1}"#,
    )
    .unwrap();
    assert_eq!(age.value(), Some(&dec("5")));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_empty_object_fails_ele1() {
    let result: Result<Age, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_value_without_code_fails_age1() {
    let result: Result<Age, _> = serde_json::from_str(r#"{"value": 5}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<Age, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}
