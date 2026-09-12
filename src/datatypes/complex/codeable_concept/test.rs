use super::*;
use crate::errors::FhirCoreError;
use crate::types::{Boolean, Code, Uri};

fn text(s: &str) -> Primitive<FhirString> {
    Primitive::from_value(FhirString::new(s).unwrap())
}

fn coding(code: &str) -> Coding {
    Coding::new(
        None,
        None,
        Some(Primitive::from_value(Code::new(code).unwrap())),
        None,
        None,
        None,
        Vec::new(),
    )
    .unwrap()
}

// --- Happy path ---

#[test]
fn test_new_with_text_only_succeeds() {
    let cc = CodeableConcept::new(Vec::new(), Some(text("Active")), None, Vec::new());
    assert!(cc.is_ok());
    let cc = cc.unwrap();
    assert_eq!(cc.text(), Some(&text("Active")));
    assert!(cc.coding().is_empty());
}

#[test]
fn test_new_with_coding_only_succeeds() {
    let cc = CodeableConcept::new(vec![coding("active")], None, None, Vec::new());
    assert!(cc.is_ok());
    let cc = cc.unwrap();
    assert_eq!(cc.coding().len(), 1);
    assert!(cc.text().is_none());
}

#[test]
fn test_new_with_multiple_codings_and_text_succeeds() {
    let cc = CodeableConcept::new(
        vec![coding("active"), coding("inactive")],
        Some(text("Active")),
        Some(FhirString::new("cc1").unwrap()),
        Vec::new(),
    );
    assert!(cc.is_ok());
    let cc = cc.unwrap();
    assert_eq!(cc.coding().len(), 2);
    assert_eq!(cc.text(), Some(&text("Active")));
    assert_eq!(cc.id().unwrap().as_str(), "cc1");
}

#[test]
fn test_new_with_extension_only_succeeds() {
    let ext = Extension::new(
        Primitive::from_value(Uri::new("http://example.org/fhir/x").unwrap()),
        None,
        Vec::new(),
        Some(crate::datatypes::complex::ExtensionValue::Boolean(
            Primitive::from_value(Boolean::new(true)),
        )),
    )
    .unwrap();
    let cc = CodeableConcept::new(Vec::new(), None, None, vec![ext]);
    assert!(cc.is_ok());
    assert_eq!(cc.unwrap().extensions().len(), 1);
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let cc = CodeableConcept::new_unchecked(Vec::new(), None, None, Vec::new());
    assert!(cc.coding().is_empty());
    assert!(cc.text().is_none());
}

// --- Negative / boundary ---

#[test]
fn test_new_with_nothing_fails_ele1() {
    let result = CodeableConcept::new(Vec::new(), None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ele-1", .. }
        ))
    ));
}

#[test]
fn test_new_with_only_id_fails_ele1() {
    let result = CodeableConcept::new(
        Vec::new(),
        None,
        Some(FhirString::new("cc1").unwrap()),
        Vec::new(),
    );
    assert!(result.is_err());
}

// --- Serde: happy path ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_coding_and_text() {
    let cc = CodeableConcept::new(
        vec![coding("active")],
        Some(text("Active")),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&cc).unwrap();
    let back: CodeableConcept = serde_json::from_str(&json).unwrap();
    assert_eq!(cc, back);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_with_id_and_extension() {
    let ext = Extension::new(
        Primitive::from_value(Uri::new("http://example.org/fhir/x").unwrap()),
        None,
        Vec::new(),
        Some(crate::datatypes::complex::ExtensionValue::Boolean(
            Primitive::from_value(Boolean::new(true)),
        )),
    )
    .unwrap();
    let cc = CodeableConcept::new(
        Vec::new(),
        Some(text("Active")),
        Some(FhirString::new("cc1").unwrap()),
        vec![ext],
    )
    .unwrap();
    let json = serde_json::to_string(&cc).unwrap();
    let back: CodeableConcept = serde_json::from_str(&json).unwrap();
    assert_eq!(cc, back);
    assert_eq!(back.id().unwrap().as_str(), "cc1");
    assert_eq!(back.extensions().len(), 1);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialize_only_populated_fields() {
    let cc = CodeableConcept::new(Vec::new(), Some(text("Active")), None, Vec::new()).unwrap();
    let json = serde_json::to_value(&cc).unwrap();
    assert_eq!(json, serde_json::json!({"text": "Active"}));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_unknown_field_ignored() {
    let cc: CodeableConcept = serde_json::from_str(r#"{"text": "Active", "unknown": 1}"#).unwrap();
    assert_eq!(cc.text(), Some(&text("Active")));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_value_with_companion() {
    let cc: CodeableConcept =
        serde_json::from_str(r#"{"text": "Active", "_text": {"id": "tid"}}"#).unwrap();
    assert_eq!(cc.text().unwrap().id().unwrap().as_str(), "tid");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_multiple_codings() {
    let cc: CodeableConcept =
        serde_json::from_str(r#"{"coding": [{"code": "active"}, {"code": "inactive"}]}"#).unwrap();
    assert_eq!(cc.coding().len(), 2);
}

// --- Serde: negative / boundary ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_empty_object_fails_ele1() {
    let result: Result<CodeableConcept, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_invalid_coding_fails() {
    // Code grammar forbids leading/trailing whitespace.
    let result: Result<CodeableConcept, _> =
        serde_json::from_str(r#"{"coding": [{"code": " bad "}]}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<CodeableConcept, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}
