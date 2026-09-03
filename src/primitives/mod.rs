//! Primitive data types defined in the FHIR specification.

#[cfg(feature = "r5")]
mod base64;
#[cfg(feature = "r5")]
mod boolean;
#[cfg(feature = "r5")]
mod decimal;
#[cfg(feature = "r5")]
mod integer;
#[cfg(feature = "r5")]
mod integer64;
#[cfg(feature = "r5")]
mod string;

// Re-exports
#[cfg(feature = "r5")]
pub use base64::Base64Binary;
#[cfg(feature = "r5")]
pub use boolean::Boolean;
#[cfg(feature = "r5")]
pub use integer::Integer;
#[cfg(feature = "r5")]
pub use integer64::Integer64;
#[cfg(feature = "r5")]
pub use string::FhirString;
