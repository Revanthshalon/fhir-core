use std::error::Error;

use super::*;
use crate::errors::r#type::TypeError;

#[test]
fn test_fhir_core_error_display() {
    let type_err = TypeError::InvalidValue {
        r#type: "boolean".to_string(),
        value: "invalid".to_string(),
        error: "expected 'true' or 'false'".to_string(),
    };
    let fhir_err = FhirCoreError::Type(type_err);
    assert_eq!(
        fhir_err.to_string(),
        "invalid value 'invalid' for type 'boolean': expected 'true' or 'false'"
    );
}

#[test]
fn test_fhir_core_error_source() {
    let type_err = TypeError::InvalidValue {
        r#type: "string".to_string(),
        value: "".to_string(),
        error: "cannot be empty".to_string(),
    };
    let fhir_err = FhirCoreError::from(type_err);
    assert!(fhir_err.source().is_some());
}
