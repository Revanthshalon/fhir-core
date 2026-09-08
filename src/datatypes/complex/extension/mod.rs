//! The FHIR `Extension` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/extensibility.html#Extension, hl7.org/fhir/R5/json.html)
//! - Fields: `id` (0..1, `string`), `extension` (0..*, `Extension` — recursive), `url`
//!   (1..1, `uri`), `value[x]` (0..1, one of 54 types: 20 primitives + 35 complex types).
//! - Invariant `ext-1` (FHIRPath: `extension.exists() != value.exists()`): a valid
//!   `Extension` has *exactly one* of nested `extension`s or a `value[x]` — never
//!   neither, never both.
//! - `url` and each `value[x]` type are primitive-valued, so both are
//!   [`Primitive<T>`](crate::datatypes::primitive::Primitive) — they may carry their own
//!   `id`/`extension` via the JSON companion pattern (`"_url": {...}`,
//!   `"_valueBoolean": {...}`). `Extension.id` itself is a plain ordinary property and
//!   never gets a `_id` companion — the spec special-cases `Element.id` this way.
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

use crate::datatypes::primitive::{
    Primitive, PrimitiveCompanion, merge_primitive_entry, serialize_primitive_entry,
};
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
    Base64Binary(Primitive<Base64Binary>),
    /// `valueBoolean`
    Boolean(Primitive<Boolean>),
    /// `valueCanonical`
    Canonical(Primitive<Canonical>),
    /// `valueCode`
    Code(Primitive<Code>),
    /// `valueDate`
    Date(Primitive<Date>),
    /// `valueDateTime`
    DateTime(Primitive<DateTime>),
    /// `valueDecimal`
    Decimal(Primitive<Decimal>),
    /// `valueId`
    Id(Primitive<Id>),
    /// `valueInstant`
    Instant(Primitive<Instant>),
    /// `valueInteger`
    Integer(Primitive<Integer>),
    /// `valueInteger64`
    Integer64(Primitive<Integer64>),
    /// `valueMarkdown`
    Markdown(Primitive<Markdown>),
    /// `valueOid`
    Oid(Primitive<Oid>),
    /// `valuePositiveInt`
    PositiveInt(Primitive<PositiveInt>),
    /// `valueString`
    String(Primitive<FhirString>),
    /// `valueTime`
    Time(Primitive<Time>),
    /// `valueUnsignedInt`
    UnsignedInt(Primitive<UnsignedInt>),
    /// `valueUri`
    Uri(Primitive<Uri>),
    /// `valueUrl`
    Url(Primitive<Url>),
    /// `valueUuid`
    Uuid(Primitive<Uuid>),
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
    url: Primitive<Uri>,
    value: Option<ExtensionValue>,
}

impl Serialize for Extension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        if let Some(id) = &self.id {
            map.serialize_entry("id", id)?;
        }
        if !self.extension.is_empty() {
            map.serialize_entry("extension", &self.extension)?;
        }
        serialize_primitive_entry(&mut map, "url", "_url", &self.url)?;
        match &self.value {
            Some(ExtensionValue::Base64Binary(p)) => {
                serialize_primitive_entry(&mut map, "valueBase64Binary", "_valueBase64Binary", p)?;
            }
            Some(ExtensionValue::Boolean(p)) => {
                serialize_primitive_entry(&mut map, "valueBoolean", "_valueBoolean", p)?;
            }
            Some(ExtensionValue::Canonical(p)) => {
                serialize_primitive_entry(&mut map, "valueCanonical", "_valueCanonical", p)?;
            }
            Some(ExtensionValue::Code(p)) => {
                serialize_primitive_entry(&mut map, "valueCode", "_valueCode", p)?;
            }
            Some(ExtensionValue::Date(p)) => {
                serialize_primitive_entry(&mut map, "valueDate", "_valueDate", p)?;
            }
            Some(ExtensionValue::DateTime(p)) => {
                serialize_primitive_entry(&mut map, "valueDateTime", "_valueDateTime", p)?;
            }
            Some(ExtensionValue::Decimal(p)) => {
                serialize_primitive_entry(&mut map, "valueDecimal", "_valueDecimal", p)?;
            }
            Some(ExtensionValue::Id(p)) => {
                serialize_primitive_entry(&mut map, "valueId", "_valueId", p)?;
            }
            Some(ExtensionValue::Instant(p)) => {
                serialize_primitive_entry(&mut map, "valueInstant", "_valueInstant", p)?;
            }
            Some(ExtensionValue::Integer(p)) => {
                serialize_primitive_entry(&mut map, "valueInteger", "_valueInteger", p)?;
            }
            Some(ExtensionValue::Integer64(p)) => {
                serialize_primitive_entry(&mut map, "valueInteger64", "_valueInteger64", p)?;
            }
            Some(ExtensionValue::Markdown(p)) => {
                serialize_primitive_entry(&mut map, "valueMarkdown", "_valueMarkdown", p)?;
            }
            Some(ExtensionValue::Oid(p)) => {
                serialize_primitive_entry(&mut map, "valueOid", "_valueOid", p)?;
            }
            Some(ExtensionValue::PositiveInt(p)) => {
                serialize_primitive_entry(&mut map, "valuePositiveInt", "_valuePositiveInt", p)?;
            }
            Some(ExtensionValue::String(p)) => {
                serialize_primitive_entry(&mut map, "valueString", "_valueString", p)?;
            }
            Some(ExtensionValue::Time(p)) => {
                serialize_primitive_entry(&mut map, "valueTime", "_valueTime", p)?;
            }
            Some(ExtensionValue::UnsignedInt(p)) => {
                serialize_primitive_entry(&mut map, "valueUnsignedInt", "_valueUnsignedInt", p)?;
            }
            Some(ExtensionValue::Uri(p)) => {
                serialize_primitive_entry(&mut map, "valueUri", "_valueUri", p)?;
            }
            Some(ExtensionValue::Url(p)) => {
                serialize_primitive_entry(&mut map, "valueUrl", "_valueUrl", p)?;
            }
            Some(ExtensionValue::Uuid(p)) => {
                serialize_primitive_entry(&mut map, "valueUuid", "_valueUuid", p)?;
            }
            None => {}
        }
        map.end()
    }
}

/// Per-type bare-value/companion accumulator pair used while scanning the JSON map.
/// One pair per `value[x]` type, since a legal document may carry either key alone
/// (e.g. `_valueBoolean` with no `valueBoolean`, the data-absent-reason pattern) or both.
struct ValueSlot<T> {
    value: Option<T>,
    companion: Option<PrimitiveCompanion>,
}

// Manual impl: `#[derive(Default)]` would require `T: Default`, which none of these
// primitive types implement (and shouldn't need to — an absent slot is just `None`).
impl<T> Default for ValueSlot<T> {
    fn default() -> Self {
        Self {
            value: None,
            companion: None,
        }
    }
}

impl<T> ValueSlot<T> {
    fn is_present(&self) -> bool {
        self.value.is_some() || self.companion.is_some()
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

            #[allow(clippy::too_many_lines)]
            fn visit_map<A>(self, mut map: A) -> Result<Extension, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut extension = Vec::new();
                let mut url: ValueSlot<Uri> = ValueSlot::default();
                let mut base64_binary: ValueSlot<Base64Binary> = ValueSlot::default();
                let mut boolean: ValueSlot<Boolean> = ValueSlot::default();
                let mut canonical: ValueSlot<Canonical> = ValueSlot::default();
                let mut code: ValueSlot<Code> = ValueSlot::default();
                let mut date: ValueSlot<Date> = ValueSlot::default();
                let mut date_time: ValueSlot<DateTime> = ValueSlot::default();
                let mut decimal: ValueSlot<Decimal> = ValueSlot::default();
                let mut id_value: ValueSlot<Id> = ValueSlot::default();
                let mut instant: ValueSlot<Instant> = ValueSlot::default();
                let mut integer: ValueSlot<Integer> = ValueSlot::default();
                let mut integer64: ValueSlot<Integer64> = ValueSlot::default();
                let mut markdown: ValueSlot<Markdown> = ValueSlot::default();
                let mut oid: ValueSlot<Oid> = ValueSlot::default();
                let mut positive_int: ValueSlot<PositiveInt> = ValueSlot::default();
                let mut string: ValueSlot<FhirString> = ValueSlot::default();
                let mut time: ValueSlot<Time> = ValueSlot::default();
                let mut unsigned_int: ValueSlot<UnsignedInt> = ValueSlot::default();
                let mut uri: ValueSlot<Uri> = ValueSlot::default();
                let mut url_value: ValueSlot<Url> = ValueSlot::default();
                let mut uuid: ValueSlot<Uuid> = ValueSlot::default();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => id = Some(map.next_value()?),
                        "extension" => extension = map.next_value()?,
                        "url" => url.value = Some(map.next_value()?),
                        "_url" => url.companion = Some(map.next_value()?),
                        "valueBase64Binary" => base64_binary.value = Some(map.next_value()?),
                        "_valueBase64Binary" => base64_binary.companion = Some(map.next_value()?),
                        "valueBoolean" => boolean.value = Some(map.next_value()?),
                        "_valueBoolean" => boolean.companion = Some(map.next_value()?),
                        "valueCanonical" => canonical.value = Some(map.next_value()?),
                        "_valueCanonical" => canonical.companion = Some(map.next_value()?),
                        "valueCode" => code.value = Some(map.next_value()?),
                        "_valueCode" => code.companion = Some(map.next_value()?),
                        "valueDate" => date.value = Some(map.next_value()?),
                        "_valueDate" => date.companion = Some(map.next_value()?),
                        "valueDateTime" => date_time.value = Some(map.next_value()?),
                        "_valueDateTime" => date_time.companion = Some(map.next_value()?),
                        "valueDecimal" => decimal.value = Some(map.next_value()?),
                        "_valueDecimal" => decimal.companion = Some(map.next_value()?),
                        "valueId" => id_value.value = Some(map.next_value()?),
                        "_valueId" => id_value.companion = Some(map.next_value()?),
                        "valueInstant" => instant.value = Some(map.next_value()?),
                        "_valueInstant" => instant.companion = Some(map.next_value()?),
                        "valueInteger" => integer.value = Some(map.next_value()?),
                        "_valueInteger" => integer.companion = Some(map.next_value()?),
                        "valueInteger64" => integer64.value = Some(map.next_value()?),
                        "_valueInteger64" => integer64.companion = Some(map.next_value()?),
                        "valueMarkdown" => markdown.value = Some(map.next_value()?),
                        "_valueMarkdown" => markdown.companion = Some(map.next_value()?),
                        "valueOid" => oid.value = Some(map.next_value()?),
                        "_valueOid" => oid.companion = Some(map.next_value()?),
                        "valuePositiveInt" => positive_int.value = Some(map.next_value()?),
                        "_valuePositiveInt" => positive_int.companion = Some(map.next_value()?),
                        "valueString" => string.value = Some(map.next_value()?),
                        "_valueString" => string.companion = Some(map.next_value()?),
                        "valueTime" => time.value = Some(map.next_value()?),
                        "_valueTime" => time.companion = Some(map.next_value()?),
                        "valueUnsignedInt" => unsigned_int.value = Some(map.next_value()?),
                        "_valueUnsignedInt" => unsigned_int.companion = Some(map.next_value()?),
                        "valueUri" => uri.value = Some(map.next_value()?),
                        "_valueUri" => uri.companion = Some(map.next_value()?),
                        "valueUrl" => url_value.value = Some(map.next_value()?),
                        "_valueUrl" => url_value.companion = Some(map.next_value()?),
                        "valueUuid" => uuid.value = Some(map.next_value()?),
                        "_valueUuid" => uuid.companion = Some(map.next_value()?),
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let url = merge_primitive_entry(url.value, url.companion)?;

                let value = if base64_binary.is_present() {
                    Some(ExtensionValue::Base64Binary(merge_primitive_entry(
                        base64_binary.value,
                        base64_binary.companion,
                    )?))
                } else if boolean.is_present() {
                    Some(ExtensionValue::Boolean(merge_primitive_entry(
                        boolean.value,
                        boolean.companion,
                    )?))
                } else if canonical.is_present() {
                    Some(ExtensionValue::Canonical(merge_primitive_entry(
                        canonical.value,
                        canonical.companion,
                    )?))
                } else if code.is_present() {
                    Some(ExtensionValue::Code(merge_primitive_entry(
                        code.value,
                        code.companion,
                    )?))
                } else if date.is_present() {
                    Some(ExtensionValue::Date(merge_primitive_entry(
                        date.value,
                        date.companion,
                    )?))
                } else if date_time.is_present() {
                    Some(ExtensionValue::DateTime(merge_primitive_entry(
                        date_time.value,
                        date_time.companion,
                    )?))
                } else if decimal.is_present() {
                    Some(ExtensionValue::Decimal(merge_primitive_entry(
                        decimal.value,
                        decimal.companion,
                    )?))
                } else if id_value.is_present() {
                    Some(ExtensionValue::Id(merge_primitive_entry(
                        id_value.value,
                        id_value.companion,
                    )?))
                } else if instant.is_present() {
                    Some(ExtensionValue::Instant(merge_primitive_entry(
                        instant.value,
                        instant.companion,
                    )?))
                } else if integer.is_present() {
                    Some(ExtensionValue::Integer(merge_primitive_entry(
                        integer.value,
                        integer.companion,
                    )?))
                } else if integer64.is_present() {
                    Some(ExtensionValue::Integer64(merge_primitive_entry(
                        integer64.value,
                        integer64.companion,
                    )?))
                } else if markdown.is_present() {
                    Some(ExtensionValue::Markdown(merge_primitive_entry(
                        markdown.value,
                        markdown.companion,
                    )?))
                } else if oid.is_present() {
                    Some(ExtensionValue::Oid(merge_primitive_entry(
                        oid.value,
                        oid.companion,
                    )?))
                } else if positive_int.is_present() {
                    Some(ExtensionValue::PositiveInt(merge_primitive_entry(
                        positive_int.value,
                        positive_int.companion,
                    )?))
                } else if string.is_present() {
                    Some(ExtensionValue::String(merge_primitive_entry(
                        string.value,
                        string.companion,
                    )?))
                } else if time.is_present() {
                    Some(ExtensionValue::Time(merge_primitive_entry(
                        time.value,
                        time.companion,
                    )?))
                } else if unsigned_int.is_present() {
                    Some(ExtensionValue::UnsignedInt(merge_primitive_entry(
                        unsigned_int.value,
                        unsigned_int.companion,
                    )?))
                } else if uri.is_present() {
                    Some(ExtensionValue::Uri(merge_primitive_entry(
                        uri.value,
                        uri.companion,
                    )?))
                } else if url_value.is_present() {
                    Some(ExtensionValue::Url(merge_primitive_entry(
                        url_value.value,
                        url_value.companion,
                    )?))
                } else if uuid.is_present() {
                    Some(ExtensionValue::Uuid(merge_primitive_entry(
                        uuid.value,
                        uuid.companion,
                    )?))
                } else {
                    None
                };

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
    /// use fhir_core::datatypes::primitive::Primitive;
    /// use fhir_core::types::{Boolean, Uri};
    ///
    /// let url = Primitive::from_value(Uri::new("http://example.org/fhir/StructureDefinition/my-flag").unwrap());
    /// let ext = Extension::new(
    ///     url,
    ///     None,
    ///     Vec::new(),
    ///     Some(ExtensionValue::Boolean(Primitive::from_value(Boolean::new(true)))),
    /// );
    /// assert!(ext.is_ok());
    /// ```
    pub fn new(
        url: Primitive<Uri>,
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
        url: Primitive<Uri>,
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
    pub fn url(&self) -> &Primitive<Uri> {
        &self.url
    }

    /// Returns the extension's value, if present.
    #[inline]
    pub fn value(&self) -> Option<&ExtensionValue> {
        self.value.as_ref()
    }
}
