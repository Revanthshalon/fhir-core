//! The FHIR `Extension` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/extensibility.html#Extension)
//! - Fields: `id` (0..1, `string`), `extension` (0..*, `Extension` — recursive), `url`
//!   (1..1, `uri`), `value[x]` (0..1, one of 54 types: 20 primitives + 35 complex types).
//! - Invariant `ext-1` (FHIRPath: `extension.exists() != value.exists()`): a valid
//!   `Extension` has *exactly one* of nested `extension`s or a `value[x]` — never
//!   neither, never both.
//!
//! # Scope
//! [`ExtensionValue`] currently covers only the 20 FHIR primitives, because those are
//! the only types this crate has implemented so far — not the full 54 the spec allows.
//! A complex-type variant (`Coding`, `Quantity`, `Period`, ...) is added the moment that
//! complex type is built, not before; `Extension` is the natural second consumer for
//! each one. See `docs/LLD.md` §4 for the rationale against building this speculatively
//! ahead of real consumers.
//!
//! # Usage
//! Use [`Extension::new`] to construct a validated instance (checks `ext-1`), or
//! [`Extension::new_unchecked`] when the fields are already known to satisfy it.

use crate::errors::{FhirCoreResult, constraints::ConstraintError};
use crate::types::{
    Base64Binary, Boolean, Canonical, Code, Date, DateTime, Decimal, FhirString, Id, Instant,
    Integer, Integer64, Markdown, Oid, PositiveInt, Time, UnsignedInt, Uri, Url, Uuid,
};

#[cfg(test)]
mod test;

/// The value carried by `Extension.value[x]`.
///
/// One variant per FHIR primitive type this crate implements today (see module docs
/// for why complex types aren't represented yet).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionValue {
    /// `valueBase64Binary`
    Base64Binary(Base64Binary),
    /// `valueBoolean`
    Boolean(Boolean),
    /// `valueCanonical`
    Canonical(Canonical),
    /// `valueCode`
    Code(Code),
    /// `valueDate`
    Date(Date),
    /// `valueDateTime`
    DateTime(DateTime),
    /// `valueDecimal`
    Decimal(Decimal),
    /// `valueId`
    Id(Id),
    /// `valueInstant`
    Instant(Instant),
    /// `valueInteger`
    Integer(Integer),
    /// `valueInteger64`
    Integer64(Integer64),
    /// `valueMarkdown`
    Markdown(Markdown),
    /// `valueOid`
    Oid(Oid),
    /// `valuePositiveInt`
    PositiveInt(PositiveInt),
    /// `valueString`
    String(FhirString),
    /// `valueTime`
    Time(Time),
    /// `valueUnsignedInt`
    UnsignedInt(UnsignedInt),
    /// `valueUri`
    Uri(Uri),
    /// `valueUrl`
    Url(Url),
    /// `valueUuid`
    Uuid(Uuid),
}

/// The FHIR `Extension` complex data type.
///
/// # Invariants
/// Any instance of `Extension` constructed via [`Extension::new`] is guaranteed to
/// satisfy `ext-1`: exactly one of `extension` (non-empty) or `value` (`Some`) is set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    url: Uri,
    value: Option<ExtensionValue>,
}

impl Extension {
    /// Creates a new `Extension`, validating the `ext-1` invariant.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Constraint`](crate::errors::FhirCoreError::Constraint)
    /// containing [`ConstraintError::InvariantViolated`] if `extension` and `value` are
    /// not exactly one-set-one-unset (both empty/`None`, or both non-empty/`Some`).
    ///
    /// # Examples
    /// ```
    /// use fhir_core::datatypes::complex::extension::{Extension, ExtensionValue};
    /// use fhir_core::types::{Boolean, Uri};
    ///
    /// let url = Uri::new("http://example.org/fhir/StructureDefinition/my-flag").unwrap();
    /// let ext = Extension::new(
    ///     url,
    ///     None,
    ///     Vec::new(),
    ///     Some(ExtensionValue::Boolean(Boolean::new(true))),
    /// );
    /// assert!(ext.is_ok());
    /// ```
    pub fn new(
        url: Uri,
        id: Option<FhirString>,
        extension: Vec<Extension>,
        value: Option<ExtensionValue>,
    ) -> FhirCoreResult<Self> {
        Self::validate_ext1(&extension, &value)?;
        Ok(Self {
            id,
            extension,
            url,
            value,
        })
    }

    /// Validates the `ext-1` invariant against candidate `extension`/`value` fields.
    fn validate_ext1(
        extension: &[Extension],
        value: &Option<ExtensionValue>,
    ) -> Result<(), ConstraintError> {
        let has_extension = !extension.is_empty();
        let has_value = value.is_some();
        // ext-1 is an XOR (`extension.exists() != value.exists()`): exactly one must be
        // set, so matching booleans (both set or both unset) is the violation.
        if has_extension == has_value {
            return Err(ConstraintError::InvariantViolated {
                key: "ext-1",
                description: "Must have either extensions or value[x], not both".to_owned(),
            });
        }
        Ok(())
    }

    /// Creates a new `Extension` without validating the `ext-1` invariant.
    ///
    /// # Warning
    /// The caller is responsible for ensuring `extension` and `value` satisfy `ext-1`
    /// (exactly one set).
    #[inline]
    pub fn new_unchecked(
        url: Uri,
        id: Option<FhirString>,
        extension: Vec<Extension>,
        value: Option<ExtensionValue>,
    ) -> Self {
        Self {
            id,
            extension,
            url,
            value,
        }
    }

    /// Returns the element-level identifier, if present.
    #[inline]
    pub fn id(&self) -> Option<&FhirString> {
        self.id.as_ref()
    }

    /// Returns the slice of nested extensions.
    #[inline]
    pub fn extensions(&self) -> &[Extension] {
        &self.extension
    }

    /// Returns the extension's URL, identifying its meaning.
    #[inline]
    pub fn url(&self) -> &Uri {
        &self.url
    }

    /// Returns the extension's value, if present.
    #[inline]
    pub fn value(&self) -> Option<&ExtensionValue> {
        self.value.as_ref()
    }
}
