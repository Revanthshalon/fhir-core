//! Error types and result aliases used throughout the crate.

pub mod constraints;
pub mod r#type;

#[cfg(test)]
mod test;

/// A specialized [`Result`] type for operations that can produce a [`FhirCoreError`].
pub type FhirCoreResult<T> = Result<T, FhirCoreError>;

use r#type::TypeError;

/// The top-level error type representing any error that can occur within `fhir-core`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FhirCoreError {
    /// An error related to FHIR primitive or complex type validation.
    Type(TypeError),
}

impl std::fmt::Display for FhirCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FhirCoreError::Type(type_error) => type_error.fmt(f),
        }
    }
}

impl std::error::Error for FhirCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FhirCoreError::Type(type_error) => Some(type_error),
        }
    }
}

impl From<TypeError> for FhirCoreError {
    fn from(value: TypeError) -> Self {
        Self::Type(value)
    }
}
