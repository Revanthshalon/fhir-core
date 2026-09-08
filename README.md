# fhir-core

[![CI](https://github.com/Revanthshalon/fhir-core/actions/workflows/ci.yml/badge.svg)](https://github.com/Revanthshalon/fhir-core/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/Revanthshalon/c87aa3a1e9742e401e70f4fad127a885/raw/fhir-core-coverage.json)](https://github.com/Revanthshalon/fhir-core/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/fhir-core.svg)](https://crates.io/crates/fhir-core)
[![Documentation](https://docs.rs/fhir-core/badge.svg)](https://docs.rs/fhir-core)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

Core primitive types, data models, and validation logic for Fast Healthcare Interoperability Resources ([FHIR](https://hl7.org/fhir/)).

`fhir-core` provides strongly typed, specification-compliant Rust representations of FHIR primitives with built-in validation, zero-cost serialization, and error handling.

---

## Features

- **Strict FHIR Compliance**: Enforces format invariants, character set restrictions, size limits, and padding rules as defined in the FHIR specification.
- **Robust Error Handling**: Granular, typed error reporting through [`TypeError`](crate::errors::r#type::TypeError) and type-specific error enums.
- **Fast & Zero-Cost Serde**: Efficient JSON serialization and deserialization via `serde` without unnecessary cloning.
- **Feature-Gated Versions**: Modular design supporting FHIR releases (e.g. `r5`).

---

## Supported Primitives

| Primitive | Rust Type | Description |
| :--- | :--- | :--- |
| `base64Binary` | [`Base64Binary`](src/types/base64/mod.rs) | RFC 4648 Base64-encoded byte streams with strict padding and alphabet validation. |
| `boolean` | [`Boolean`](src/types/boolean/mod.rs) | Binary `true` or `false` value. |
| `decimal` | [`Decimal`](src/types/decimal/mod.rs) | Arbitrary-precision decimal preserving trailing-zero precision (e.g. `0.010`). |
| `id` | [`Id`](src/types/id/mod.rs) | Resource-local identifier, `[A-Za-z0-9\-\.]{1,64}`. |
| `integer` | [`Integer`](src/types/integer/mod.rs) | 32-bit signed integer, `-2^31 .. 2^31-1`. |
| `integer64` | [`Integer64`](src/types/integer64/mod.rs) | 64-bit signed integer, serialized as a JSON string per spec. |
| `string` | [`FhirString`](src/types/string/mod.rs) | Unicode strings up to 1,048,576 characters, with control character and whitespace validation. |

---

## Installation

Add `fhir-core` to your `Cargo.toml`:

```toml
[dependencies]
fhir-core = "0.1.0"
```

Or using `cargo`:

```bash
cargo add fhir-core
```

---

## Usage Examples

### Base64Binary

```rust
use fhir_core::types::Base64Binary;

// Valid Base64 data
let b64 = Base64Binary::new("TWFu").unwrap();
assert_eq!(b64.as_str(), "TWFu");

// Invalid Base64 strings fail validation
assert!(Base64Binary::new("TWF").is_err()); // Invalid length
assert!(Base64Binary::new("TW@u").is_err()); // Invalid character
```

### Boolean

```rust
use fhir_core::types::Boolean;

let b = Boolean::new(true);
assert!(b.as_bool());

// Parse from string slice
let from_str = Boolean::try_from("true").unwrap();
assert_eq!(from_str, Boolean::new(true));

// Invalid strings fail validation
assert!(Boolean::try_from("yes").is_err());
```

### Decimal

```rust
use fhir_core::types::Decimal;

// Trailing zeros are significant in FHIR and are preserved verbatim
let d = Decimal::try_from("0.010").unwrap();
assert_eq!(d.as_str(), "0.010");
assert_eq!(d.as_f64(), 0.01);

// Invalid decimals fail validation
assert!(Decimal::try_from("01.5").is_err()); // Leading zero
assert!(Decimal::try_from("+1.5").is_err()); // Leading '+' not allowed
```

### Id

```rust
use fhir_core::types::Id;

let id = Id::new("patient-1234").unwrap();
assert_eq!(id.as_str(), "patient-1234");

// Invalid ids fail validation
assert!(Id::new("").is_err()); // Empty
assert!(Id::new("has a space").is_err()); // Disallowed character
```

### Integer & Integer64

```rust
use fhir_core::types::{Integer, Integer64};

let i = Integer::try_from("42").unwrap();
assert_eq!(i32::from(i), 42);

// integer64 is serialized as a JSON string per the FHIR R5 spec
let i64_val = Integer64::try_from("9223372036854775807").unwrap();
let json = serde_json::to_string(&i64_val).unwrap();
assert_eq!(json, "\"9223372036854775807\"");
```

### String

```rust
use fhir_core::types::FhirString;

// Valid string
let s = FhirString::new("Patient record notes").unwrap();
assert_eq!(s.as_str(), "Patient record notes");

// Validation rejects empty strings, whitespace-only strings, or disallowed control characters
assert!(FhirString::new("").is_err());
assert!(FhirString::new("   \t\n  ").is_err());
assert!(FhirString::new("bad\0char").is_err());
```

### JSON Serialization & Deserialization

```rust
use fhir_core::types::{Base64Binary, Boolean, FhirString};
use serde_json;

// Serialization
let s = FhirString::new("Observation Note").unwrap();
let json = serde_json::to_string(&s).unwrap();
assert_eq!(json, "\"Observation Note\"");

// Deserialization with validation
let deserialized: FhirString = serde_json::from_str(&json).unwrap();
assert_eq!(deserialized.as_str(), "Observation Note");
```

---

## Feature Flags

| Feature | Default | Description |
| :--- | :---: | :--- |
| `r5` | **Yes** | Enables FHIR Release 5 (R5) models and primitives. |
| `serde` | **Yes** | Enables `serde` serialization and deserialization support. |

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
