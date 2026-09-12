//! Primitive data types defined in the FHIR specification.

#[cfg(any(feature = "r4", feature = "r5"))]
mod base64;
#[cfg(any(feature = "r4", feature = "r5"))]
mod boolean;
#[cfg(any(feature = "r4", feature = "r5"))]
mod canonical;
#[cfg(any(feature = "r4", feature = "r5"))]
mod code;
#[cfg(any(feature = "r4", feature = "r5"))]
mod date;
#[cfg(any(feature = "r4", feature = "r5"))]
mod date_time;
#[cfg(any(feature = "r4", feature = "r5"))]
mod decimal;
#[cfg(any(feature = "r4", feature = "r5"))]
mod id;
#[cfg(any(feature = "r4", feature = "r5"))]
mod instant;
#[cfg(any(feature = "r4", feature = "r5"))]
mod integer;
#[cfg(feature = "r5")]
mod integer64;
#[cfg(any(feature = "r4", feature = "r5"))]
mod markdown;
#[cfg(any(feature = "r4", feature = "r5"))]
mod oid;
#[cfg(any(feature = "r4", feature = "r5"))]
mod positive_int;
#[cfg(any(feature = "r4", feature = "r5"))]
mod string;
#[cfg(any(feature = "r4", feature = "r5"))]
mod time;
#[cfg(any(feature = "r4", feature = "r5"))]
mod unsigned_int;
#[cfg(any(feature = "r4", feature = "r5"))]
mod uri;
#[cfg(any(feature = "r4", feature = "r5"))]
mod url;
#[cfg(any(feature = "r4", feature = "r5"))]
mod uuid;

// Re-exports
#[cfg(any(feature = "r4", feature = "r5"))]
pub use base64::Base64Binary;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use boolean::Boolean;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use canonical::Canonical;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use code::Code;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use date::Date;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use date_time::DateTime;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use decimal::Decimal;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use id::Id;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use instant::Instant;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use integer::Integer;
#[cfg(feature = "r5")]
pub use integer64::Integer64;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use markdown::Markdown;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use oid::Oid;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use positive_int::PositiveInt;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use string::FhirString;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use time::Time;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use unsigned_int::UnsignedInt;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use uri::Uri;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use url::Url;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use uuid::Uuid;

// Error re-exports. Each primitive's `validate`/`TryFrom` can surface its granular
// error enum directly (not just the crate-wide `TypeError` it also maps into) — these
// must be re-exported or external callers receive a value whose type they can't name,
// import, or pattern-match on.
#[cfg(any(feature = "r4", feature = "r5"))]
pub use base64::Base64Error;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use canonical::CanonicalError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use code::CodeError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use date::DateError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use date_time::DateTimeError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use decimal::DecimalError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use id::IdError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use instant::InstantError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use integer::IntegerError;
#[cfg(feature = "r5")]
pub use integer64::Integer64Error;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use markdown::MarkdownError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use oid::OidError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use positive_int::PositiveIntError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use string::StringError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use time::TimeError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use unsigned_int::UnsignedIntError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use uri::UriError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use url::UrlError;
#[cfg(any(feature = "r4", feature = "r5"))]
pub use uuid::UuidError;
