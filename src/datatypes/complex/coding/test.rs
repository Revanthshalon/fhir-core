use super::*;
use crate::errors::FhirCoreError;

fn code(s: &str) -> Primitive<Code> {
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
fn test_new_with_code_only_succeeds() {
    let coding = Coding::new(
        None,
        None,
        Some(code("active")),
        None,
        None,
        None,
        Vec::new(),
    );
    assert!(coding.is_ok());
    let coding = coding.unwrap();
    assert_eq!(coding.code(), Some(&code("active")));
    assert!(coding.system().is_none());
}

#[test]
fn test_new_with_all_fields_succeeds() {
    let coding = Coding::new(
        Some(uri("http://terminology.hl7.org/CodeSystem/v3-ActCode")),
        Some(text("2018-08")),
        Some(code("active")),
        Some(text("Active")),
        Some(Primitive::from_value(Boolean::new(true))),
        Some(FhirString::new("c1").unwrap()),
        Vec::new(),
    );
    assert!(coding.is_ok());
    let coding = coding.unwrap();
    assert_eq!(
        coding.system(),
        Some(&uri("http://terminology.hl7.org/CodeSystem/v3-ActCode"))
    );
    assert_eq!(coding.version(), Some(&text("2018-08")));
    assert_eq!(coding.code(), Some(&code("active")));
    assert_eq!(coding.display(), Some(&text("Active")));
    assert_eq!(
        coding.user_selected(),
        Some(&Primitive::from_value(Boolean::new(true)))
    );
    assert_eq!(coding.id().unwrap().as_str(), "c1");
}

#[test]
fn test_new_display_without_code_succeeds_cod1_is_warning_only() {
    // cod-1 is a warning-severity invariant (SHOULD NOT), not a SHALL — this crate
    // only rejects construction for error-severity invariants.
    let coding = Coding::new(
        None,
        None,
        None,
        Some(text("Active")),
        None,
        None,
        Vec::new(),
    );
    assert!(coding.is_ok());
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
    assert!(ext.value().is_some());
    let coding = Coding::new(None, None, None, None, None, None, vec![ext]);
    assert!(coding.is_ok());
    assert_eq!(coding.unwrap().extensions().len(), 1);
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let coding = Coding::new_unchecked(None, None, None, None, None, None, Vec::new());
    assert!(coding.code().is_none());
}

// --- Negative / boundary ---

#[test]
fn test_new_with_nothing_fails_ele1() {
    let result = Coding::new(None, None, None, None, None, None, Vec::new());
    assert!(matches!(
        result,
        Err(FhirCoreError::Constraint(
            crate::errors::constraints::ConstraintError::InvariantViolated { key: "ele-1", .. }
        ))
    ));
}

#[test]
fn test_new_with_only_id_fails_ele1() {
    // id alone does not count as a child satisfying ele-1.
    let result = Coding::new(
        None,
        None,
        None,
        None,
        None,
        Some(FhirString::new("c1").unwrap()),
        Vec::new(),
    );
    assert!(result.is_err());
}

// --- Serde: happy path ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_all_fields() {
    let coding = Coding::new(
        Some(uri("http://terminology.hl7.org/CodeSystem/v3-ActCode")),
        Some(text("2018-08")),
        Some(code("active")),
        Some(text("Active")),
        Some(Primitive::from_value(Boolean::new(true))),
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_string(&coding).unwrap();
    let back: Coding = serde_json::from_str(&json).unwrap();
    assert_eq!(coding, back);
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
    let coding = Coding::new(
        None,
        None,
        Some(code("active")),
        None,
        None,
        Some(FhirString::new("c1").unwrap()),
        vec![ext],
    )
    .unwrap();
    let json = serde_json::to_string(&coding).unwrap();
    let back: Coding = serde_json::from_str(&json).unwrap();
    assert_eq!(coding, back);
    assert_eq!(back.id().unwrap().as_str(), "c1");
    assert_eq!(back.extensions().len(), 1);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialize_only_populated_fields() {
    let coding = Coding::new(
        None,
        None,
        Some(code("active")),
        None,
        None,
        None,
        Vec::new(),
    )
    .unwrap();
    let json = serde_json::to_value(&coding).unwrap();
    assert_eq!(json, serde_json::json!({"code": "active"}));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_unknown_field_ignored() {
    let coding: Coding = serde_json::from_str(r#"{"code": "active", "unknown": 1}"#).unwrap();
    assert_eq!(coding.code(), Some(&code("active")));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_value_with_companion() {
    let coding: Coding =
        serde_json::from_str(r#"{"code": "active", "_code": {"id": "cid"}}"#).unwrap();
    assert_eq!(coding.code().unwrap().id().unwrap().as_str(), "cid");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_companion_only_no_bare_value() {
    // data-absent-reason style: only the companion is present.
    let coding: Coding = serde_json::from_str(
        r#"{"_display": {"extension": [{"url": "http://example.org/fhir/x", "valueBoolean": true}]}}"#,
    )
    .unwrap();
    assert!(coding.display().is_some());
    assert!(coding.display().unwrap().value().is_none());
}

// --- Serde: negative / boundary ---

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_empty_object_fails_ele1() {
    let result: Result<Coding, _> = serde_json::from_str("{}");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_invalid_code_fails() {
    // Code grammar forbids leading/trailing whitespace.
    let result: Result<Coding, _> = serde_json::from_str(r#"{"code": " bad "}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_invalid_uri_system_fails() {
    let result: Result<Coding, _> = serde_json::from_str(r#"{"system": "not a uri"}"#);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_non_object_fails() {
    let result: Result<Coding, _> = serde_json::from_str(r#""just a string""#);
    assert!(result.is_err());
}
