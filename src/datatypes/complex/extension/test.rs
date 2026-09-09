use super::*;
use crate::datatypes::primitive::Primitive;
use crate::errors::FhirCoreError;

fn url(s: &str) -> Primitive<Uri> {
    Primitive::from_value(Uri::new(s).unwrap())
}

#[test]
fn test_new_with_value_only_succeeds() {
    let ext = Extension::new(
        url("http://example.org/fhir/StructureDefinition/flag"),
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(true),
        ))),
    );
    assert!(ext.is_ok());
    let ext = ext.unwrap();
    assert!(ext.extensions().is_empty());
    assert_eq!(
        ext.value(),
        Some(&ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(true)
        )))
    );
}

#[test]
fn test_new_with_children_only_succeeds() {
    let child = Extension::new(
        url("http://example.org/fhir/StructureDefinition/child"),
        None,
        Vec::new(),
        Some(ExtensionValue::String(Primitive::from_value(
            FhirString::new("hello").unwrap(),
        ))),
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
        Some(ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(false),
        ))),
    )
    .unwrap();

    let result = Extension::new(
        url("http://example.org/fhir/StructureDefinition/both"),
        None,
        vec![child],
        Some(ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(true),
        ))),
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
        Some(ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(true),
        ))),
    )
    .unwrap();

    assert_eq!(ext.id().unwrap().as_str(), "ext-1-id");
    assert_eq!(
        ext.url().value().unwrap().as_str(),
        "http://example.org/fhir/StructureDefinition/flag"
    );
}

#[test]
fn test_nested_extension_recursion() {
    let grandchild = Extension::new(
        url("http://example.org/fhir/StructureDefinition/grandchild"),
        None,
        Vec::new(),
        Some(ExtensionValue::Integer(Primitive::from_value(
            Integer::new(1),
        ))),
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
        Some(&ExtensionValue::Integer(Primitive::from_value(
            Integer::new(1)
        )))
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
fn test_url_with_extension_only_no_value() {
    // ele-1 permits a Primitive<T> with extension but no value; url's own companion
    // can legally carry this even though url itself is a mandatory (1..1) field.
    let note = Extension::new(
        url("http://example.org/fhir/StructureDefinition/note"),
        None,
        Vec::new(),
        Some(ExtensionValue::String(Primitive::from_value(
            FhirString::new("why").unwrap(),
        ))),
    )
    .unwrap();
    let url_only_ext = Primitive::new(None, None, vec![note]).unwrap();

    let ext = Extension::new(
        url_only_ext,
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(true),
        ))),
    )
    .unwrap();
    assert!(ext.url().value().is_none());
    assert_eq!(ext.url().extensions().len(), 1);
}

fn all_extension_values() -> Vec<ExtensionValue> {
    vec![
        ExtensionValue::Base64Binary(Primitive::from_value(Base64Binary::new("TWFu").unwrap())),
        ExtensionValue::Boolean(Primitive::from_value(Boolean::new(true))),
        ExtensionValue::Canonical(Primitive::from_value(
            Canonical::new("http://example.org").unwrap(),
        )),
        ExtensionValue::Code(Primitive::from_value(Code::new("active").unwrap())),
        ExtensionValue::Date(Primitive::from_value(Date::new("2020-01-01").unwrap())),
        ExtensionValue::DateTime(Primitive::from_value(
            DateTime::new("2020-01-01T00:00:00Z").unwrap(),
        )),
        ExtensionValue::Decimal(Primitive::from_value(Decimal::try_from("1.5").unwrap())),
        ExtensionValue::Id(Primitive::from_value(Id::new("abc-123").unwrap())),
        ExtensionValue::Instant(Primitive::from_value(
            Instant::new("2020-01-01T00:00:00Z").unwrap(),
        )),
        ExtensionValue::Integer(Primitive::from_value(Integer::new(42))),
        ExtensionValue::Integer64(Primitive::from_value(Integer64::new(42))),
        ExtensionValue::Markdown(Primitive::from_value(Markdown::new("**bold**").unwrap())),
        ExtensionValue::Oid(Primitive::from_value(Oid::new("urn:oid:1.2.3").unwrap())),
        ExtensionValue::PositiveInt(Primitive::from_value(PositiveInt::new(1).unwrap())),
        ExtensionValue::String(Primitive::from_value(FhirString::new("hello").unwrap())),
        ExtensionValue::Time(Primitive::from_value(Time::new("12:00:00").unwrap())),
        ExtensionValue::UnsignedInt(Primitive::from_value(UnsignedInt::new(0).unwrap())),
        ExtensionValue::Uri(Primitive::from_value(
            Uri::new("http://example.org").unwrap(),
        )),
        ExtensionValue::Url(Primitive::from_value(
            Url::new("http://example.org").unwrap(),
        )),
        ExtensionValue::Uuid(Primitive::from_value(
            Uuid::new("urn:uuid:c757873d-ec9a-4326-a141-556f43239520").unwrap(),
        )),
    ]
}

#[test]
fn test_extension_value_variants_roundtrip() {
    for value in all_extension_values() {
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

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialize_value_only() {
    let ext = Extension::new(
        url("http://example.org/fhir/StructureDefinition/flag"),
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(true),
        ))),
    )
    .unwrap();
    let json = serde_json::to_string(&ext).unwrap();
    assert_eq!(
        json,
        r#"{"url":"http://example.org/fhir/StructureDefinition/flag","valueBoolean":true}"#
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialize_with_id_and_children() {
    let child = Extension::new(
        url("http://example.org/fhir/StructureDefinition/child"),
        None,
        Vec::new(),
        Some(ExtensionValue::String(Primitive::from_value(
            FhirString::new("hi").unwrap(),
        ))),
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

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialize_value_with_companion() {
    let note = Extension::new(
        url("http://example.org/fhir/StructureDefinition/note"),
        None,
        Vec::new(),
        Some(ExtensionValue::String(Primitive::from_value(
            FhirString::new("why unknown").unwrap(),
        ))),
    )
    .unwrap();
    let boolean_with_ext = Primitive::new(Some(Boolean::new(true)), None, vec![note]).unwrap();

    let ext = Extension::new(
        url("http://example.org/fhir/StructureDefinition/flag"),
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(boolean_with_ext)),
    )
    .unwrap();
    let json = serde_json::to_string(&ext).unwrap();
    assert_eq!(
        json,
        r#"{"url":"http://example.org/fhir/StructureDefinition/flag","valueBoolean":true,"_valueBoolean":{"extension":[{"url":"http://example.org/fhir/StructureDefinition/note","valueString":"why unknown"}]}}"#
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialize_companion_only_no_value() {
    let note = Extension::new(
        url("http://example.org/fhir/StructureDefinition/data-absent-reason"),
        None,
        Vec::new(),
        Some(ExtensionValue::Code(Primitive::from_value(
            Code::new("unknown").unwrap(),
        ))),
    )
    .unwrap();
    let value_absent = Primitive::new(None, None, vec![note]).unwrap();

    let ext = Extension::new(
        url("http://example.org/fhir/StructureDefinition/flag"),
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(value_absent)),
    )
    .unwrap();
    let json = serde_json::to_string(&ext).unwrap();
    assert_eq!(
        json,
        r#"{"url":"http://example.org/fhir/StructureDefinition/flag","_valueBoolean":{"extension":[{"url":"http://example.org/fhir/StructureDefinition/data-absent-reason","valueCode":"unknown"}]}}"#
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_value_only() {
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/flag","valueBoolean":true}"#;
    let ext: Extension = serde_json::from_str(json).unwrap();
    assert_eq!(
        ext.url().value().unwrap().as_str(),
        "http://example.org/fhir/StructureDefinition/flag"
    );
    assert_eq!(
        ext.value(),
        Some(&ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(true)
        )))
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_value_with_companion() {
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/flag","valueBoolean":true,"_valueBoolean":{"id":"b1"}}"#;
    let ext: Extension = serde_json::from_str(json).unwrap();
    let ExtensionValue::Boolean(p) = ext.value().unwrap() else {
        panic!("expected Boolean variant");
    };
    assert_eq!(p.value(), Some(&Boolean::new(true)));
    assert_eq!(p.id().unwrap().as_str(), "b1");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_companion_only_no_bare_value() {
    // id alone would violate ele-1 (id doesn't count, per the FHIRPath nuance), so the
    // companion needs an extension to be a legal "value absent" state.
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/flag","_valueString":{"extension":[{"url":"http://example.org/fhir/StructureDefinition/reason","valueCode":"unknown"}]}}"#;
    let ext: Extension = serde_json::from_str(json).unwrap();
    let ExtensionValue::String(p) = ext.value().unwrap() else {
        panic!("expected String variant");
    };
    assert!(p.value().is_none());
    assert_eq!(p.extensions().len(), 1);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_id_alone_on_companion_fails_ele1() {
    // Spec nuance: id alone (no value, no extension) does not satisfy ele-1.
    let json =
        r#"{"url":"http://example.org/fhir/StructureDefinition/flag","_valueString":{"id":"s1"}}"#;
    let result: Result<Extension, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_url_companion() {
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/flag","_url":{"id":"u1"},"valueBoolean":true}"#;
    let ext: Extension = serde_json::from_str(json).unwrap();
    assert_eq!(
        ext.url().value().unwrap().as_str(),
        "http://example.org/fhir/StructureDefinition/flag"
    );
    assert_eq!(ext.url().id().unwrap().as_str(), "u1");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_nested_children() {
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/parent","extension":[{"url":"http://example.org/fhir/StructureDefinition/child","valueInteger":42}]}"#;
    let ext: Extension = serde_json::from_str(json).unwrap();
    assert_eq!(ext.extensions().len(), 1);
    assert_eq!(
        ext.extensions()[0].value(),
        Some(&ExtensionValue::Integer(Primitive::from_value(
            Integer::new(42)
        )))
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_missing_url_fails() {
    let json = r#"{"valueBoolean":true}"#;
    let result: Result<Extension, _> = serde_json::from_str(json);
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("missing field") && err.contains("url"),
        "expected a missing-field error naming `url`, got: {err}"
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_multiple_value_x_fails() {
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/x","valueBoolean":true,"valueString":"bar"}"#;
    let result: Result<Extension, _> = serde_json::from_str(json);
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("more than one value[x]"),
        "expected a multiple-value[x] error, got: {err}"
    );
}

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_unknown_field_ignored() {
    let json = r#"{"url":"http://example.org/fhir/StructureDefinition/flag","valueBoolean":true,"unknownField":"ignored"}"#;
    let ext: Extension = serde_json::from_str(json).unwrap();
    assert_eq!(
        ext.value(),
        Some(&ExtensionValue::Boolean(Primitive::from_value(
            Boolean::new(true)
        )))
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_all_variants() {
    for value in all_extension_values() {
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

#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip_with_companion() {
    let note = Extension::new(
        url("http://example.org/fhir/StructureDefinition/note"),
        None,
        Vec::new(),
        Some(ExtensionValue::String(Primitive::from_value(
            FhirString::new("hi").unwrap(),
        ))),
    )
    .unwrap();
    let boolean_with_id_and_ext = Primitive::new(
        Some(Boolean::new(true)),
        Some(FhirString::new("b1").unwrap()),
        vec![note],
    )
    .unwrap();

    let original = Extension::new(
        url("http://example.org/fhir/StructureDefinition/flag"),
        None,
        Vec::new(),
        Some(ExtensionValue::Boolean(boolean_with_id_and_ext)),
    )
    .unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Extension = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}
