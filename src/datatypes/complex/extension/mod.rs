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

use serde::de::{Error as DeError, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

impl Serialize for Extension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = usize::from(self.id.is_some())
            + usize::from(!self.extension.is_empty())
            + 1 // url
            + usize::from(self.value.is_some());
        let mut map = serializer.serialize_map(Some(len))?;
        if let Some(id) = &self.id {
            map.serialize_entry("id", id)?;
        }
        if !self.extension.is_empty() {
            map.serialize_entry("extension", &self.extension)?;
        }
        map.serialize_entry("url", &self.url)?;
        match &self.value {
            Some(ExtensionValue::Base64Binary(v)) => map.serialize_entry("valueBase64Binary", v)?,
            Some(ExtensionValue::Boolean(v)) => map.serialize_entry("valueBoolean", v)?,
            Some(ExtensionValue::Canonical(v)) => map.serialize_entry("valueCanonical", v)?,
            Some(ExtensionValue::Code(v)) => map.serialize_entry("valueCode", v)?,
            Some(ExtensionValue::Date(v)) => map.serialize_entry("valueDate", v)?,
            Some(ExtensionValue::DateTime(v)) => map.serialize_entry("valueDateTime", v)?,
            Some(ExtensionValue::Decimal(v)) => map.serialize_entry("valueDecimal", v)?,
            Some(ExtensionValue::Id(v)) => map.serialize_entry("valueId", v)?,
            Some(ExtensionValue::Instant(v)) => map.serialize_entry("valueInstant", v)?,
            Some(ExtensionValue::Integer(v)) => map.serialize_entry("valueInteger", v)?,
            Some(ExtensionValue::Integer64(v)) => map.serialize_entry("valueInteger64", v)?,
            Some(ExtensionValue::Markdown(v)) => map.serialize_entry("valueMarkdown", v)?,
            Some(ExtensionValue::Oid(v)) => map.serialize_entry("valueOid", v)?,
            Some(ExtensionValue::PositiveInt(v)) => map.serialize_entry("valuePositiveInt", v)?,
            Some(ExtensionValue::String(v)) => map.serialize_entry("valueString", v)?,
            Some(ExtensionValue::Time(v)) => map.serialize_entry("valueTime", v)?,
            Some(ExtensionValue::UnsignedInt(v)) => map.serialize_entry("valueUnsignedInt", v)?,
            Some(ExtensionValue::Uri(v)) => map.serialize_entry("valueUri", v)?,
            Some(ExtensionValue::Url(v)) => map.serialize_entry("valueUrl", v)?,
            Some(ExtensionValue::Uuid(v)) => map.serialize_entry("valueUuid", v)?,
            None => {}
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Extension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExtensionVisitor;

        impl<'de> Visitor<'de> for ExtensionVisitor {
            type Value = Extension;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FHIR Extension JSON object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Extension, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut extension = Vec::new();
                let mut url = None;
                let mut value = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => id = Some(map.next_value()?),
                        "extension" => extension = map.next_value()?,
                        "url" => url = Some(map.next_value()?),
                        "valueBase64Binary" => {
                            value = Some(ExtensionValue::Base64Binary(map.next_value()?));
                        }
                        "valueBoolean" => value = Some(ExtensionValue::Boolean(map.next_value()?)),
                        "valueCanonical" => {
                            value = Some(ExtensionValue::Canonical(map.next_value()?));
                        }
                        "valueCode" => value = Some(ExtensionValue::Code(map.next_value()?)),
                        "valueDate" => value = Some(ExtensionValue::Date(map.next_value()?)),
                        "valueDateTime" => {
                            value = Some(ExtensionValue::DateTime(map.next_value()?));
                        }
                        "valueDecimal" => value = Some(ExtensionValue::Decimal(map.next_value()?)),
                        "valueId" => value = Some(ExtensionValue::Id(map.next_value()?)),
                        "valueInstant" => value = Some(ExtensionValue::Instant(map.next_value()?)),
                        "valueInteger" => value = Some(ExtensionValue::Integer(map.next_value()?)),
                        "valueInteger64" => {
                            value = Some(ExtensionValue::Integer64(map.next_value()?));
                        }
                        "valueMarkdown" => {
                            value = Some(ExtensionValue::Markdown(map.next_value()?));
                        }
                        "valueOid" => value = Some(ExtensionValue::Oid(map.next_value()?)),
                        "valuePositiveInt" => {
                            value = Some(ExtensionValue::PositiveInt(map.next_value()?));
                        }
                        "valueString" => value = Some(ExtensionValue::String(map.next_value()?)),
                        "valueTime" => value = Some(ExtensionValue::Time(map.next_value()?)),
                        "valueUnsignedInt" => {
                            value = Some(ExtensionValue::UnsignedInt(map.next_value()?));
                        }
                        "valueUri" => value = Some(ExtensionValue::Uri(map.next_value()?)),
                        "valueUrl" => value = Some(ExtensionValue::Url(map.next_value()?)),
                        "valueUuid" => value = Some(ExtensionValue::Uuid(map.next_value()?)),
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let url = url.ok_or_else(|| DeError::missing_field("url"))?;
                Extension::new(url, id, extension, value).map_err(DeError::custom)
            }
        }

        deserializer.deserialize_map(ExtensionVisitor)
    }
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
