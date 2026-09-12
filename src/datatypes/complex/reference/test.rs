use super::*;
use crate::errors::FhirCoreError;
use crate::types::Boolean;

fn text(s: &str) -> Primitive<FhirString> {
    Primitive::from_value(FhirString::new(s).unwrap())
}

fn uri(s: &str) -> Primitive<Uri> {
    Primitive::from_value(Uri::new(s).unwrap())
}

fn simple_identifier() -> Identifier {
    Identifier::new(
        None,
        None,
        None,
        Some(text("12345")),
        None,
        None,
        None,
        Vec::new(),
    )
    .unwrap()
}

// --- Happy path ---

#[test]
fn test_new_with_reference_only_succeeds() {
    let r = Reference::new(
        Some(text("Patient/123")),
        None,
        None,
        None,
        None,
        Vec::new(),
    );
    assert!(r.is_ok());
    let r = r.unwrap();
    assert_eq!(r.reference(), Some(&text("Patient/123")));
    assert!(r.display().is_none());
}

#[test]
fn test_new_with_all_fields_succeeds() {
    let r = Reference::new(
        Some(text("Patient/123")),
        Some(uri("Patient")),
        Some(Box::new(simple_identifier())),
        Some(text("Jane Doe")),
        Some(FhirString::new("r1").unwrap()),
        Vec::new(),
    );
    assert!(r.is_ok());
    let r = r.unwrap();
    assert_eq!(r.reference(), Some(&text("Patient/123")));
    assert_eq!(r.r#type(), Some(&uri("Patient")));
    assert_eq!(r.identifier(), Some(&simple_identifier()));
    assert_eq!(r.display(), Some(&text("Jane Doe")));
    assert_eq!(r.id().unwrap().as_str(), "r1");
}

#[test]
fn test_new_with_display_only_succeeds() {
    let r = Reference::new(None, None, None, Some(text("Jane Doe")), None, Vec::new());
    assert!(r.is_ok());
}

#[test]
fn test_new_with_identifier_only_succeeds() {
    let r = Reference::new(
        None,
        None,
        Some(Box::new(simple_identifier())),
        None,
        None,
        Vec::new(),
    );
    assert!(r.is_ok());
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
    let r = Reference::new(None, None, None, None, None, vec![ext]);
    assert!(r.is_ok());
    assert_eq!(r.unwrap().extensions().len(), 1);
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let r = Reference::new_unchecked(None, None, None, None, None, Vec::new());
    assert!(r.reference().is_none());
}

// --- Negative / boundary ---

#[test]
fn test_new_with_nothing_fails_ref2() {
    let result = Reference::new(None, None, None, None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ref-2", .. }
        ))
    ));
}

#[test]
fn test_new_with_only_type_fails_ref2() {
    // type alone does not satisfy ref-2, unlike a generic ele-1 check.
    let result = Reference::new(None, Some(uri("Patient")), None, None, None, Vec::new());
    assert!(result.is_err());
}

#[test]
fn test_new_with_only_id_fails_ref2() {
    let result = Reference::new(
        None,
        None,
        None,
        None,
        Some(FhirString::new("r1").unwrap()),
        Vec::new(),
    );
    assert!(result.is_err());
}

#[test]
fn test_new_unchecked_bypasses_ref2() {
    let r = Reference::new_unchecked(None, Some(uri("Patient")), None, None, None, Vec::new());
    assert_eq!(r.r#type(), Some(&uri("Patient")));
    assert!(r.reference().is_none());
}

// --- Serde: happy path ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_all_fields() {
    let r = Reference::new(
        Some(text("Patient/123")),
        Some(uri("Patient")),
        Some(Box::new(simple_identifier())),
        Some(text("Jane Doe")),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&r).unwrap();
    let back: Reference = serde_json::from_str(&json).unwrap();
    assert_eq!(r, back);
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
    let r = Reference::new(
        Some(text("Patient/123")),
        None,
        None,
        None,
        Some(FhirString::new("r1").unwrap()),
        vec![ext],
    )
    .unwrap();
    let json = serde_json::to_string(&r).unwrap();
    let back: Reference = serde_json::from_str(&json).unwrap();
    assert_eq!(r, back);
    assert_eq!(back.id().unwrap().as_str(), "r1");
    assert_eq!(back.extensions().len(), 1);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialize_only_populated_fields() {
    let r = Reference::new(
        Some(text("Patient/123")),
        None,
        None,
        None,
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_value(&r).unwrap();
    assert_eq!(json, serde_json::json!({"reference": "Patient/123"}));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_unknown_field_ignored() {
    let r: Reference =
        serde_json::from_str(r#"{"reference": "Patient/123", "unknown": 1}"#).unwrap();
    assert_eq!(r.reference(), Some(&text("Patient/123")));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_value_with_companion() {
    let r: Reference =
        serde_json::from_str(r#"{"reference": "Patient/123", "_reference": {"id": "rid"}}"#)
            .unwrap();
    assert_eq!(r.reference().unwrap().id().unwrap().as_str(), "rid");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_nested_identifier() {
    let r: Reference = serde_json::from_str(r#"{"identifier": {"value": "12345"}}"#).unwrap();
    assert!(r.identifier().is_some());
}

// --- Serde: negative / boundary ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_empty_object_fails_ref2() {
    let result: Result<Reference, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_type_only_fails_ref2() {
    let result: Result<Reference, _> = serde_json::from_str(r#"{"type": "Patient"}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_invalid_type_uri_fails() {
    let result: Result<Reference, _> =
        serde_json::from_str(r#"{"reference": "Patient/123", "type": "not a uri"}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_invalid_nested_identifier_fails() {
    // Nested Identifier must itself satisfy ele-1.
    let result: Result<Reference, _> = serde_json::from_str(r#"{"identifier": {}}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<Reference, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}
