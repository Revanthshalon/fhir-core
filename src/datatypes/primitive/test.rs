use super::*;
use crate::errors::FhirCoreError;
use crate::types::{Boolean, FhirString as Str, Uri};

fn ext(url: &str) -> Extension {
    Extension::new(
        Primitive::from_value(Uri::new(url).unwrap()),
        None,
        Vec::new(),
        Some(crate::datatypes::complex::ExtensionValue::Boolean(
            Primitive::from_value(Boolean::new(true)),
        )),
    )
    .unwrap()
}

#[test]
fn test_new_with_value_only_succeeds() {
    let p = Primitive::new(Some(Boolean::new(true)), None, Vec::new());
    assert!(p.is_ok());
    let p = p.unwrap();
    assert_eq!(p.value(), Some(&Boolean::new(true)));
    assert!(p.id().is_none());
    assert!(p.extensions().is_empty());
}

#[test]
fn test_new_with_extension_only_succeeds() {
    let p = Primitive::new(
        None::<Boolean>,
        None,
        vec![ext(
            "http://example.org/fhir/StructureDefinition/data-absent-reason",
        )],
    );
    assert!(p.is_ok());
    let p = p.unwrap();
    assert!(p.value().is_none());
    assert_eq!(p.extensions().len(), 1);
}

#[test]
fn test_new_with_value_and_extension_succeeds() {
    let p = Primitive::new(
        Some(Boolean::new(true)),
        None,
        vec![ext("http://example.org/fhir/StructureDefinition/note")],
    );
    assert!(p.is_ok());
}

#[test]
fn test_new_with_neither_value_nor_extension_fails_ele1() {
    let result = Primitive::new(None::<Boolean>, None, Vec::new());
    assert!(result.is_err());
    match result.unwrap_err() {
        FhirCoreError::Constraint(err) => {
            assert_eq!(
                err.to_string(),
                "invariant 'ele-1' violated: All FHIR elements must have a @value or children"
            );
        }
        other => panic!("expected Constraint error, got {other:?}"),
    }
}

#[test]
fn test_id_alone_does_not_satisfy_ele1() {
    // Spec nuance: id counts as a child in the ele-1 FHIRPath, so id alone (no value,
    // no extension) is NOT sufficient — this is the exact case ele-1 forbids.
    let result = Primitive::new(
        None::<Boolean>,
        Some(Str::new("just-an-id").unwrap()),
        Vec::new(),
    );
    assert!(result.is_err());
}

#[test]
fn test_from_value_is_infallible_and_bare() {
    let p = Primitive::from_value(Boolean::new(true));
    assert_eq!(p.value(), Some(&Boolean::new(true)));
    assert!(p.id().is_none());
    assert!(p.extensions().is_empty());
}

#[test]
fn test_new_unchecked_bypasses_validation() {
    let p: Primitive<Boolean> = Primitive::new_unchecked(None, None, Vec::new());
    assert!(p.value().is_none());
    assert!(p.extensions().is_empty());
}

#[test]
fn test_into_value() {
    let p = Primitive::from_value(Boolean::new(true));
    assert_eq!(p.into_value(), Some(Boolean::new(true)));
}

#[test]
fn test_id_with_value_present() {
    let p = Primitive::new(
        Some(Boolean::new(true)),
        Some(Str::new("b1").unwrap()),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(p.id().unwrap().as_str(), "b1");
    assert_eq!(p.value(), Some(&Boolean::new(true)));
}

#[test]
fn test_clone_and_equality() {
    let a = Primitive::from_value(Boolean::new(true));
    let b = a.clone();
    assert_eq!(a, b);
    let c = Primitive::from_value(Boolean::new(false));
    assert_ne!(a, c);
}
