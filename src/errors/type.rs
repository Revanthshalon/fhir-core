//! Type-level validation error representations.

/// Errors encountered when parsing or validating FHIR data types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    /// The value provided does not conform to the expected format for the FHIR type.
    InvalidValue {
        /// The name of the FHIR data type (e.g. `"base64Binary"`).
        r#type: String,
        /// The string representation of the invalid value that was provided.
        value: String,
        /// A description of the specific validation error encountered.
        error: String,
    },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::InvalidValue {
                r#type,
                value,
                error,
            } => write!(f, "invalid value '{value}' for type '{type}': {error}"),
        }
    }
}

impl std::error::Error for TypeError {}
