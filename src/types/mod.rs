//! Primitive data types defined in the FHIR specification.

#[cfg(feature = "r5")]
mod base64;
#[cfg(feature = "r5")]
mod boolean;
#[cfg(feature = "r5")]
mod canonical;
#[cfg(feature = "r5")]
mod code;
#[cfg(feature = "r5")]
mod date;
#[cfg(feature = "r5")]
mod date_time;
#[cfg(feature = "r5")]
mod decimal;
#[cfg(feature = "r5")]
mod id;
#[cfg(feature = "r5")]
mod instant;
#[cfg(feature = "r5")]
mod integer;
#[cfg(feature = "r5")]
mod integer64;
#[cfg(feature = "r5")]
mod markdown;
#[cfg(feature = "r5")]
mod oid;
#[cfg(feature = "r5")]
mod positive_int;
#[cfg(feature = "r5")]
mod string;
#[cfg(feature = "r5")]
mod time;
#[cfg(feature = "r5")]
mod unsigned_int;
#[cfg(feature = "r5")]
mod uri;
#[cfg(feature = "r5")]
mod url;
#[cfg(feature = "r5")]
mod uuid;

// Re-exports
#[cfg(feature = "r5")]
pub use base64::Base64Binary;
#[cfg(feature = "r5")]
pub use boolean::Boolean;
#[cfg(feature = "r5")]
pub use canonical::Canonical;
#[cfg(feature = "r5")]
pub use code::Code;
#[cfg(feature = "r5")]
pub use date::Date;
#[cfg(feature = "r5")]
pub use date_time::DateTime;
#[cfg(feature = "r5")]
pub use decimal::Decimal;
#[cfg(feature = "r5")]
pub use id::Id;
#[cfg(feature = "r5")]
pub use instant::Instant;
#[cfg(feature = "r5")]
pub use integer::Integer;
#[cfg(feature = "r5")]
pub use integer64::Integer64;
#[cfg(feature = "r5")]
pub use markdown::Markdown;
#[cfg(feature = "r5")]
pub use oid::Oid;
#[cfg(feature = "r5")]
pub use positive_int::PositiveInt;
#[cfg(feature = "r5")]
pub use string::FhirString;
#[cfg(feature = "r5")]
pub use time::Time;
#[cfg(feature = "r5")]
pub use unsigned_int::UnsignedInt;
#[cfg(feature = "r5")]
pub use uri::Uri;
#[cfg(feature = "r5")]
pub use url::Url;
#[cfg(feature = "r5")]
pub use uuid::Uuid;
