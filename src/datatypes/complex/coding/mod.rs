//! The FHIR `Coding` complex data type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/datatypes.html#Coding,
//! hl7.org/fhir/R5/datatypes-definitions.html#Coding.userSelected)
//! - Fields: `system` (0..1, `uri` — the code system that defines the code's meaning),
//!   `version` (0..1, `string` — the code system version in use when this code was
//!   chosen), `code` (0..1, `code`), `display` (0..1, `string` — human-readable
//!   rendering of the code's meaning), `userSelected` (0..1, `boolean` — was this
//!   coding chosen directly by a user).
//! - Invariant `cod-1` (FHIRPath: `code.exists().not() implies display.exists().not()`,
//!   **Warning** severity): "A Coding SHOULD NOT have a display unless a code is also
//!   present." Not enforced here — it's a *warning*-severity invariant (SHOULD), unlike
//!   `ext-1`/`ele-1`/`per-1` which are *error*-severity (SHALL). This crate's
//!   `ConstraintError::InvariantViolated` models hard SHALL failures that block
//!   construction; a `display`-without-`code` `Coding` is spec-legal (if
//!   discouraged), so `Coding::new` accepts it. Add a non-fatal warning channel only if
//!   a real caller needs one — not speculatively for this single case.
//! - Like every FHIR type, `Coding` inherits `id` (0..1, `string`) and `extension`
//!   (0..*, `Extension`) from `Element`, and is therefore itself subject to `ele-1`
//!   ("must have a value or children") — with no primitive `value` of its own, at least
//!   one of `system`/`version`/`code`/`display`/`userSelected`/`extension` must be
//!   present.
//! - Every value field is primitive-valued, so each is
//!   [`Primitive<T>`](crate::datatypes::primitive::Primitive) per the JSON companion
//!   pattern (see `Extension`'s module docs). `Coding.id` is a plain ordinary property,
//!   never itself companion-wrapped, matching `Extension.id`/`Period.id`.
//!
//! # Usage
//! Use [`Coding::new`] to construct a validated instance (checks `ele-1`), or
//! [`Coding::new_unchecked`] when the fields are already known to satisfy it.

#[cfg(feature = "serde")]
use serde::de::{Error as DeError, MapAccess, Visitor};
#[cfg(feature = "serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::datatypes::complex::Extension;
use crate::datatypes::primitive::Primitive;
#[cfg(feature = "serde")]
use crate::datatypes::primitive::{
    PrimitiveCompanion, merge_primitive_entry, serialize_primitive_entry,
};
use crate::errors::{FhirCoreResult, constraints::ConstraintError};
use crate::types::{Boolean, Code, FhirString, Uri};

#[cfg(test)]
mod test;

/// The FHIR `Coding` complex data type: a single code from a code system, optionally
/// with a human-readable display and version/user-selection metadata.
///
/// # Invariants
/// Any instance of `Coding` constructed via [`Coding::new`] is guaranteed to satisfy
/// `ele-1`: at least one of `system`, `version`, `code`, `display`, `userSelected`, or
/// `extension` is set. See the module docs for why `cod-1` (a warning-severity
/// invariant) is not validated here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coding {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    system: Option<Primitive<Uri>>,
    version: Option<Primitive<FhirString>>,
    code: Option<Primitive<Code>>,
    display: Option<Primitive<FhirString>>,
    user_selected: Option<Primitive<Boolean>>,
}

#[cfg(feature = "serde")]
impl Serialize for Coding {
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
        if let Some(system) = &self.system {
            serialize_primitive_entry(&mut map, "system", "_system", system)?;
        }
        if let Some(version) = &self.version {
            serialize_primitive_entry(&mut map, "version", "_version", version)?;
        }
        if let Some(code) = &self.code {
            serialize_primitive_entry(&mut map, "code", "_code", code)?;
        }
        if let Some(display) = &self.display {
            serialize_primitive_entry(&mut map, "display", "_display", display)?;
        }
        if let Some(user_selected) = &self.user_selected {
            serialize_primitive_entry(&mut map, "userSelected", "_userSelected", user_selected)?;
        }
        map.end()
    }
}

/// Per-type bare-value/companion accumulator pair used while scanning the JSON map. See
/// `Extension`'s `ValueSlot` for the fuller explanation this mirrors.
#[cfg(feature = "serde")]
struct Slot<T> {
    value: Option<T>,
    companion: Option<PrimitiveCompanion>,
}

// Manual impl: `#[derive(Default)]` would require `T: Default`, which none of these
// primitive types implement (and shouldn't need to).
#[cfg(feature = "serde")]
impl<T> Default for Slot<T> {
    fn default() -> Self {
        Self {
            value: None,
            companion: None,
        }
    }
}

#[cfg(feature = "serde")]
impl<T> Slot<T> {
    fn is_present(&self) -> bool {
        self.value.is_some() || self.companion.is_some()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Coding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CodingVisitor;

        impl<'de> Visitor<'de> for CodingVisitor {
            type Value = Coding;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FHIR Coding JSON object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Coding, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut extension = Vec::new();
                let mut system: Slot<Uri> = Slot::default();
                let mut version: Slot<FhirString> = Slot::default();
                let mut code: Slot<Code> = Slot::default();
                let mut display: Slot<FhirString> = Slot::default();
                let mut user_selected: Slot<Boolean> = Slot::default();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => id = Some(map.next_value()?),
                        "extension" => extension = map.next_value()?,
                        "system" => system.value = Some(map.next_value()?),
                        "_system" => system.companion = Some(map.next_value()?),
                        "version" => version.value = Some(map.next_value()?),
                        "_version" => version.companion = Some(map.next_value()?),
                        "code" => code.value = Some(map.next_value()?),
                        "_code" => code.companion = Some(map.next_value()?),
                        "display" => display.value = Some(map.next_value()?),
                        "_display" => display.companion = Some(map.next_value()?),
                        "userSelected" => user_selected.value = Some(map.next_value()?),
                        "_userSelected" => user_selected.companion = Some(map.next_value()?),
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let system = system
                    .is_present()
                    .then(|| merge_primitive_entry(system.value, system.companion))
                    .transpose()?;
                let version = version
                    .is_present()
                    .then(|| merge_primitive_entry(version.value, version.companion))
                    .transpose()?;
                let code = code
                    .is_present()
                    .then(|| merge_primitive_entry(code.value, code.companion))
                    .transpose()?;
                let display = display
                    .is_present()
                    .then(|| merge_primitive_entry(display.value, display.companion))
                    .transpose()?;
                let user_selected = user_selected
                    .is_present()
                    .then(|| merge_primitive_entry(user_selected.value, user_selected.companion))
                    .transpose()?;

                Coding::new(system, version, code, display, user_selected, id, extension)
                    .map_err(DeError::custom)
            }
        }

        deserializer.deserialize_map(CodingVisitor)
    }
}

impl Coding {
    /// Creates a new `Coding`, validating the `ele-1` invariant.
    ///
    /// # Errors
    /// Returns [`FhirCoreError::Constraint`](crate::errors::FhirCoreError::Constraint)
    /// containing [`ConstraintError::InvariantViolated`] if `system`, `version`, `code`,
    /// `display`, `userSelected`, and `extension` are all absent/empty.
    ///
    /// # Examples
    /// ```
    /// use fhir_core::datatypes::complex::coding::Coding;
    /// use fhir_core::datatypes::primitive::Primitive;
    /// use fhir_core::types::Code;
    ///
    /// let code = Primitive::from_value(Code::new("active").unwrap());
    /// let coding = Coding::new(None, None, Some(code), None, None, None, Vec::new());
    /// assert!(coding.is_ok());
    ///
    /// let empty = Coding::new(None, None, None, None, None, None, Vec::new());
    /// assert!(empty.is_err());
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        system: Option<Primitive<Uri>>,
        version: Option<Primitive<FhirString>>,
        code: Option<Primitive<Code>>,
        display: Option<Primitive<FhirString>>,
        user_selected: Option<Primitive<Boolean>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> FhirCoreResult<Self> {
        Self::validate_ele1(
            &system,
            &version,
            &code,
            &display,
            &user_selected,
            &extension,
        )?;
        Ok(Self {
            id,
            extension,
            system,
            version,
            code,
            display,
            user_selected,
        })
    }

    /// Validates the `ele-1` invariant against candidate fields.
    #[allow(clippy::too_many_arguments)]
    fn validate_ele1(
        system: &Option<Primitive<Uri>>,
        version: &Option<Primitive<FhirString>>,
        code: &Option<Primitive<Code>>,
        display: &Option<Primitive<FhirString>>,
        user_selected: &Option<Primitive<Boolean>>,
        extension: &[Extension],
    ) -> Result<(), ConstraintError> {
        if system.is_none()
            && version.is_none()
            && code.is_none()
            && display.is_none()
            && user_selected.is_none()
            && extension.is_empty()
        {
            return Err(ConstraintError::InvariantViolated {
                key: "ele-1",
                description: "All FHIR elements must have a @value or children".to_owned(),
            });
        }
        Ok(())
    }

    /// Creates a new `Coding` without validating the `ele-1` invariant.
    ///
    /// # Warning
    /// The caller is responsible for ensuring at least one field satisfies `ele-1`.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new_unchecked(
        system: Option<Primitive<Uri>>,
        version: Option<Primitive<FhirString>>,
        code: Option<Primitive<Code>>,
        display: Option<Primitive<FhirString>>,
        user_selected: Option<Primitive<Boolean>>,
        id: Option<FhirString>,
        extension: Vec<Extension>,
    ) -> Self {
        Self {
            id,
            extension,
            system,
            version,
            code,
            display,
            user_selected,
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

    /// Returns the code system, if present.
    #[inline]
    pub fn system(&self) -> Option<&Primitive<Uri>> {
        self.system.as_ref()
    }

    /// Returns the code system version, if present.
    #[inline]
    pub fn version(&self) -> Option<&Primitive<FhirString>> {
        self.version.as_ref()
    }

    /// Returns the code, if present.
    #[inline]
    pub fn code(&self) -> Option<&Primitive<Code>> {
        self.code.as_ref()
    }

    /// Returns the human-readable display, if present.
    #[inline]
    pub fn display(&self) -> Option<&Primitive<FhirString>> {
        self.display.as_ref()
    }

    /// Returns whether this coding was chosen directly by a user, if specified.
    #[inline]
    pub fn user_selected(&self) -> Option<&Primitive<Boolean>> {
        self.user_selected.as_ref()
    }
}
