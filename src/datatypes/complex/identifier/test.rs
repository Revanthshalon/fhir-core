use super::*;
use crate::errors::FhirCoreError;
use crate::types::{Boolean, DateTime};

fn value(s: &str) -> Primitive<FhirString> {
    Primitive::from_value(FhirString::new(s).unwrap())
}

fn use_code(s: &str) -> Primitive<Code> {
    Primitive::from_value(Code::new(s).unwrap())
}

fn sys(s: &str) -> Primitive<Uri> {
    Primitive::from_value(Uri::new(s).unwrap())
}

fn simple_type() -> CodeableConcept {
    CodeableConcept::new(Vec::new(), Some(value("MRN")), None, Vec::new()).unwrap()
}

fn simple_period() -> Period {
    Period::new(
        Some(Primitive::from_value(
            DateTime::new("2024-01-01T00:00:00Z").unwrap(),
        )),
        None,
        None,
        Vec::new(),
    )
    .unwrap()
}

fn simple_reference() -> Reference {
    Reference::new(
        Some(value("Organization/1")),
        None,
        None,
        None,
        None,
        Vec::new(),
    )
    .unwrap()
}

// --- Happy path ---

#[test]
fn test_new_with_value_only_succeeds() {
    let id = Identifier::new(
        None,
        None,
        None,
        Some(value("12345")),
        None,
        None,
        None,
        Vec::new(),
    );
    assert!(id.is_ok());
    let id = id.unwrap();
    assert_eq!(id.value(), Some(&value("12345")));
    assert!(id.r#use().is_none());
}

#[test]
fn test_new_with_all_fields_succeeds() {
    let id = Identifier::new(
        Some(use_code("official")),
        Some(simple_type()),
        Some(sys("http://example.org/mrn")),
        Some(value("12345")),
        Some(simple_period()),
        Some(Box::new(simple_reference())),
        Some(FhirString::new("i1").unwrap()),
        Vec::new(),
    );
    assert!(id.is_ok());
    let id = id.unwrap();
    assert_eq!(id.r#use(), Some(&use_code("official")));
    assert_eq!(id.r#type(), Some(&simple_type()));
    assert_eq!(id.system(), Some(&sys("http://example.org/mrn")));
    assert_eq!(id.value(), Some(&value("12345")));
    assert_eq!(id.period(), Some(&simple_period()));
    assert_eq!(id.assigner(), Some(&simple_reference()));
    assert_eq!(id.id().unwrap().as_str(), "i1");
}

#[test]
fn test_new_with_extension_only_succeeds() {
    let ext = Extension::new(
        sys("http://example.org/fhir/x"),
        None,
        Vec::new(),
        Some(crate::datatypes::complex::ExtensionValue::Boolean(
            Primitive::from_value(Boolean::new(true)),
        )),
    )
    .unwrap();
    let id = Identifier::new(None, None, None, None, None, None, None, vec![ext]);
    assert!(id.is_ok());
    assert_eq!(id.unwrap().extensions().len(), 1);
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let id = Identifier::new_unchecked(None, None, None, None, None, None, None, Vec::new());
    assert!(id.value().is_none());
}

// --- Negative / boundary ---

#[test]
fn test_new_with_nothing_fails_ele1() {
    let result = Identifier::new(None, None, None, None, None, None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ele-1", .. }
        ))
    ));
}

#[test]
fn test_new_with_only_id_fails_ele1() {
    let result = Identifier::new(
        None,
        None,
        None,
        None,
        None,
        None,
        Some(FhirString::new("i1").unwrap()),
        Vec::new(),
    );
    assert!(result.is_err());
}

#[test]
fn test_new_with_no_value_succeeds_ident1_is_warning_only() {
    // ident-1 (value SHOULD be present) is warning-severity -- not enforced.
    let id = Identifier::new(
        Some(use_code("official")),
        None,
        None,
        None,
        None,
        None,
        None,
        Vec::new(),
    );
    assert!(id.is_ok());
}

// --- Serde: happy path ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_all_fields() {
    let id = Identifier::new(
        Some(use_code("official")),
        Some(simple_type()),
        Some(sys("http://example.org/mrn")),
        Some(value("12345")),
        Some(simple_period()),
        Some(Box::new(simple_reference())),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&id).unwrap();
    let back: Identifier = serde_json::from_str(&json).unwrap();
    assert_eq!(id, back);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_with_id_and_extension() {
    let ext = Extension::new(
        sys("http://example.org/fhir/x"),
        None,
        Vec::new(),
        Some(crate::datatypes::complex::ExtensionValue::Boolean(
            Primitive::from_value(Boolean::new(true)),
        )),
    )
    .unwrap();
    let id = Identifier::new(
        None,
        None,
        None,
        Some(value("12345")),
        None,
        None,
        Some(FhirString::new("i1").unwrap()),
        vec![ext],
    )
    .unwrap();
    let json = serde_json::to_string(&id).unwrap();
    let back: Identifier = serde_json::from_str(&json).unwrap();
    assert_eq!(id, back);
    assert_eq!(back.id().unwrap().as_str(), "i1");
    assert_eq!(back.extensions().len(), 1);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialize_only_populated_fields() {
    let id = Identifier::new(
        None,
        None,
        None,
        Some(value("12345")),
        None,
        None,
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_value(&id).unwrap();
    assert_eq!(json, serde_json::json!({"value": "12345"}));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_unknown_field_ignored() {
    let id: Identifier = serde_json::from_str(r#"{"value": "12345", "unknown": 1}"#).unwrap();
    assert_eq!(id.value(), Some(&value("12345")));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_value_with_companion() {
    let id: Identifier =
        serde_json::from_str(r#"{"value": "12345", "_value": {"id": "vid"}}"#).unwrap();
    assert_eq!(id.value().unwrap().id().unwrap().as_str(), "vid");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_nested_assigner() {
    let id: Identifier =
        serde_json::from_str(r#"{"value": "12345", "assigner": {"display": "Acme Labs"}}"#)
            .unwrap();
    assert!(id.assigner().is_some());
}

// --- Serde: negative / boundary ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_empty_object_fails_ele1() {
    let result: Result<Identifier, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_invalid_use_code_fails() {
    let result: Result<Identifier, _> = serde_json::from_str(r#"{"use": " bad "}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_invalid_nested_assigner_fails() {
    // Reference's ref-2 requires at least one of reference/identifier/display/extension.
    let result: Result<Identifier, _> =
        serde_json::from_str(r#"{"value": "12345", "assigner": {}}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<Identifier, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}
