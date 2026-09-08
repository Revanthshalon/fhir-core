//! FHIR constraint and invariant evaluation errors.

/// Errors raised when a multi-field FHIR invariant (e.g. `ext-1`, `per-1`) is violated.
///
/// Distinct from [`TypeError`](crate::errors::type::TypeError), which covers a single
/// primitive/datatype's own grammar; a `ConstraintError` covers a rule that spans more
/// than one field of a complex type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintError {
    /// A named FHIR invariant was violated.
    InvariantViolated {
        /// The invariant's key as published in the spec (e.g. `"ext-1"`, `"per-1"`).
        key: &'static str,
        /// A human-readable description of what the invariant requires.
        description: String,
    },
}

impl std::fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstraintError::InvariantViolated { key, description } => {
                write!(f, "invariant '{key}' violated: {description}")
            }
        }
    }
}

impl std::error::Error for ConstraintError {}
