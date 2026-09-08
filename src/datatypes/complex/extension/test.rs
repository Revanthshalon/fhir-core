use super::*;
use crate::errors::FhirCoreError;

fn url(s: &str) -> Uri {
    Uri::new(s).unwrap()
}

#[test]
fn test_new_with_value_only_succeeds() {
    let ext = Extension::new(
        url("http://example.org/fhir/StructureDefinition/flag"),
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(Boolean::new(true))),
    );
    assert!(ext.is_ok());
    let ext = ext.unwrap();
    assert!(ext.extensions().is_empty());
    assert_eq!(
        ext.value(),
        Some(&ExtensionValue::Boolean(Boolean::new(true)))
    );
}

#[test]
fn test_new_with_children_only_succeeds() {
    let child = Extension::new(
        url("http://example.org/fhir/StructureDefinition/child"),
        None,
        Vec::new(),
        Some(ExtensionValue::String(FhirString::new("hello").unwrap())),
    )
    .unwrap();

    let ext = Extension::new(
        url("http://example.org/fhir/StructureDefinition/parent"),
        None,
        vec![child],
        None,
    );
    assert!(ext.is_ok());
    let ext = ext.unwrap();
    assert_eq!(ext.extensions().len(), 1);
    assert!(ext.value().is_none());
}

#[test]
fn test_new_with_neither_fails_ext1() {
    let result = Extension::new(
        url("http://example.org/fhir/StructureDefinition/empty"),
        None,
        Vec::new(),
        None,
    );
    assert!(result.is_err());
    match result.unwrap_err() {
        FhirCoreError::Constraint(err) => {
            assert_eq!(
                err.to_string(),
                "invariant 'ext-1' violated: Must have either extensions or value[x], not both"
            );
        }
        other => panic!("expected Constraint error, got {other:?}"),
    }
}

#[test]
fn test_new_with_both_fails_ext1() {
    let child = Extension::new(
        url("http://example.org/fhir/StructureDefinition/child"),
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(Boolean::new(false))),
    )
    .unwrap();

    let result = Extension::new(
        url("http://example.org/fhir/StructureDefinition/both"),
        None,
        vec![child],
        Some(ExtensionValue::Boolean(Boolean::new(true))),
    );
    assert!(result.is_err());
    match result.unwrap_err() {
        FhirCoreError::Constraint(err) => {
            assert_eq!(
                err.to_string(),
                "invariant 'ext-1' violated: Must have either extensions or value[x], not both"
            );
        }
        other => panic!("expected Constraint error, got {other:?}"),
    }
}

#[test]
fn test_id_and_url_accessors() {
    let ext = Extension::new(
        url("http://example.org/fhir/StructureDefinition/flag"),
        Some(FhirString::new("ext-1-id").unwrap()),
        Vec::new(),
        Some(ExtensionValue::Boolean(Boolean::new(true))),
    )
    .unwrap();

    assert_eq!(ext.id().unwrap().as_str(), "ext-1-id");
    assert_eq!(
        ext.url().as_str(),
        "http://example.org/fhir/StructureDefinition/flag"
    );
}

#[test]
fn test_nested_extension_recursion() {
    let grandchild = Extension::new(
        url("http://example.org/fhir/StructureDefinition/grandchild"),
        None,
        Vec::new(),
        Some(ExtensionValue::Integer(Integer::new(1))),
    )
    .unwrap();

    let child = Extension::new(
        url("http://example.org/fhir/StructureDefinition/child"),
        None,
        vec![grandchild],
        None,
    )
    .unwrap();

    let parent = Extension::new(
        url("http://example.org/fhir/StructureDefinition/parent"),
        None,
        vec![child],
        None,
    )
    .unwrap();

    assert_eq!(parent.extensions().len(), 1);
    assert_eq!(parent.extensions()[0].extensions().len(), 1);
    assert_eq!(
        parent.extensions()[0].extensions()[0].value(),
        Some(&ExtensionValue::Integer(Integer::new(1)))
    );
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    // Deliberately violates ext-1 (both empty): new_unchecked allows it, matching
    // every other primitive's escape hatch for trusted/internal construction.
    let ext = Extension::new_unchecked(
        url("http://example.org/fhir/StructureDefinition/empty"),
        None,
        Vec::new(),
        None,
    );
    assert!(ext.extensions().is_empty());
    assert!(ext.value().is_none());
}

#[test]
fn test_extension_value_variants_roundtrip() {
    let values = vec![
        ExtensionValue::Base64Binary(Base64Binary::new("TWFu").unwrap()),
        ExtensionValue::Boolean(Boolean::new(true)),
        ExtensionValue::Canonical(Canonical::new("http://example.org").unwrap()),
        ExtensionValue::Code(Code::new("active").unwrap()),
        ExtensionValue::Date(Date::new("2020-01-01").unwrap()),
        ExtensionValue::DateTime(DateTime::new("2020-01-01T00:00:00Z").unwrap()),
        ExtensionValue::Decimal(Decimal::try_from("1.5").unwrap()),
        ExtensionValue::Id(Id::new("abc-123").unwrap()),
        ExtensionValue::Instant(Instant::new("2020-01-01T00:00:00Z").unwrap()),
        ExtensionValue::Integer(Integer::new(42)),
        ExtensionValue::Integer64(Integer64::new(42)),
        ExtensionValue::Markdown(Markdown::new("**bold**").unwrap()),
        ExtensionValue::Oid(Oid::new("urn:oid:1.2.3").unwrap()),
        ExtensionValue::PositiveInt(PositiveInt::new(1).unwrap()),
        ExtensionValue::String(FhirString::new("hello").unwrap()),
        ExtensionValue::Time(Time::new("12:00:00").unwrap()),
        ExtensionValue::UnsignedInt(UnsignedInt::new(0).unwrap()),
        ExtensionValue::Uri(Uri::new("http://example.org").unwrap()),
        ExtensionValue::Url(Url::new("http://example.org").unwrap()),
        ExtensionValue::Uuid(Uuid::new("urn:uuid:c757873d-ec9a-4326-a141-556f43239520").unwrap()),
    ];

    for value in values {
        let ext = Extension::new(
            url("http://example.org/fhir/StructureDefinition/x"),
            None,
            Vec::new(),
            Some(value.clone()),
        )
        .unwrap();
        assert_eq!(ext.value(), Some(&value));
    }
}

#[test]
fn test_serde_serialize_value_only() {
    let ext = Extension::new(
        url("http://example.org/fhir/StructureDefinition/flag"),
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(Boolean::new(true))),
    )
    .unwrap();
    let json = serde_json::to_string(&ext).unwrap();
    assert_eq!(
        json,
        r#"{"url":"http://example.org/fhir/StructureDefinition/flag","valueBoolean":true}"#
    );
}

#[test]
fn test_serde_serialize_with_id_and_children() {
    let child = Extension::new(
        url("http://example.org/fhir/StructureDefinition/child"),
        None,
        Vec::new(),
        Some(ExtensionValue::String(FhirString::new("hi").unwrap())),
    )
    .unwrap();
    let parent = Extension::new(
        url("http://example.org/fhir/StructureDefinition/parent"),
        Some(FhirString::new("p1").unwrap()),
        vec![child],
        None,
    )
    .unwrap();
    let json = serde_json::to_string(&parent).unwrap();
    assert_eq!(
        json,
        r#"{"id":"p1","extension":[{"url":"http://example.org/fhir/StructureDefinition/child","valueString":"hi"}],"url":"http://example.org/fhir/StructureDefinition/parent"}"#
    );
}

#[test]
fn test_serde_deserialize_value_only() {
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/flag","valueBoolean":true}"#;
    let ext: Extension = serde_json::from_str(json).unwrap();
    assert_eq!(
        ext.url().as_str(),
        "http://example.org/fhir/StructureDefinition/flag"
    );
    assert_eq!(
        ext.value(),
        Some(&ExtensionValue::Boolean(Boolean::new(true)))
    );
}

#[test]
fn test_serde_deserialize_nested_children() {
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/parent","extension":[{"url":"http://example.org/fhir/StructureDefinition/child","valueInteger":42}]}"#;
    let ext: Extension = serde_json::from_str(json).unwrap();
    assert_eq!(ext.extensions().len(), 1);
    assert_eq!(
        ext.extensions()[0].value(),
        Some(&ExtensionValue::Integer(Integer::new(42)))
    );
}

#[test]
fn test_serde_deserialize_missing_url_fails() {
    let json = r#"{"valueBoolean":true}"#;
    let result: Result<Extension, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_serde_deserialize_ext1_violation_fails() {
    // Neither extension nor value present.
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/empty"}"#;
    let result: Result<Extension, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // Both present.
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/both","valueBoolean":true,"extension":[{"url":"http://example.org/fhir/StructureDefinition/child","valueInteger":1}]}"#;
    let result: Result<Extension, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_serde_deserialize_unknown_field_ignored() {
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/flag","valueBoolean":true,"unknownField":"ignored"}"#;
    let ext: Extension = serde_json::from_str(json).unwrap();
    assert_eq!(
        ext.value(),
        Some(&ExtensionValue::Boolean(Boolean::new(true)))
    );
}

#[test]
fn test_serde_roundtrip_all_variants() {
    let values = vec![
        ExtensionValue::Base64Binary(Base64Binary::new("TWFu").unwrap()),
        ExtensionValue::Boolean(Boolean::new(true)),
        ExtensionValue::Canonical(Canonical::new("http://example.org").unwrap()),
        ExtensionValue::Code(Code::new("active").unwrap()),
        ExtensionValue::Date(Date::new("2020-01-01").unwrap()),
        ExtensionValue::DateTime(DateTime::new("2020-01-01T00:00:00Z").unwrap()),
        ExtensionValue::Decimal(Decimal::try_from("1.5").unwrap()),
        ExtensionValue::Id(Id::new("abc-123").unwrap()),
        ExtensionValue::Instant(Instant::new("2020-01-01T00:00:00Z").unwrap()),
        ExtensionValue::Integer(Integer::new(42)),
        ExtensionValue::Integer64(Integer64::new(42)),
        ExtensionValue::Markdown(Markdown::new("**bold**").unwrap()),
        ExtensionValue::Oid(Oid::new("urn:oid:1.2.3").unwrap()),
        ExtensionValue::PositiveInt(PositiveInt::new(1).unwrap()),
        ExtensionValue::String(FhirString::new("hello").unwrap()),
        ExtensionValue::Time(Time::new("12:00:00").unwrap()),
        ExtensionValue::UnsignedInt(UnsignedInt::new(0).unwrap()),
        ExtensionValue::Uri(Uri::new("http://example.org").unwrap()),
        ExtensionValue::Url(Url::new("http://example.org").unwrap()),
        ExtensionValue::Uuid(Uuid::new("urn:uuid:c757873d-ec9a-4326-a141-556f43239520").unwrap()),
    ];

    for value in values {
        let original = Extension::new(
            url("http://example.org/fhir/StructureDefinition/x"),
            None,
            Vec::new(),
            Some(value),
        )
        .unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Extension = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
