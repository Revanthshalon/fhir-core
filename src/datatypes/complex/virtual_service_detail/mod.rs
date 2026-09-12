//! The FHIR `VirtualServiceDetail` metadata type.
//!
//! # FHIR R5 specification (hl7.org/fhir/R5/metadatatypes.html#VirtualServiceDetail)
//! - Fields: `channelType` (0..1, [`Coding`](crate::datatypes::complex::coding::Coding)),
//!   `address[x]` (0..1, choice of `url`, `string`,
//!   [`ContactPoint`](crate::datatypes::complex::contact_point::ContactPoint), or
//!   [`ExtendedContactDetail`](crate::datatypes::complex::extended_contact_detail::ExtendedContactDetail)),
//!   `additionalInfo` (0..*, `url`), `maxParticipants` (0..1, `positiveInt`),
//!   `sessionKey` (0..1, `string`), plus `id`/`extension`.
//! - No named invariants found.
//! - `address[x]` is a genuine FHIR choice type — represented as a local enum
//!   ([`VirtualServiceAddress`]), same pattern as `Annotation.author[x]`.
//!
//! # Status
//! Stub only — not wired into [`complex`](crate::datatypes::complex) yet. See
//! `docs/BACKLOG.md` Roadmap before implementing: re-verify against the live spec,
//! then add the constructor, accessors, and serde. Depends on `Coding`,
//! `ContactPoint`, `ExtendedContactDetail`.

#![allow(dead_code)] // stub: not constructed until this type is implemented, see module docs

use crate::datatypes::complex::Extension;
use crate::datatypes::complex::coding::Coding;
use crate::datatypes::complex::contact_point::ContactPoint;
use crate::datatypes::complex::extended_contact_detail::ExtendedContactDetail;
use crate::datatypes::primitive::Primitive;
use crate::types::{FhirString, PositiveInt, Url};

/// The value carried by `VirtualServiceDetail.address[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualServiceAddress {
    /// `addressUrl`
    Url(Primitive<Url>),
    /// `addressString`
    String(Primitive<FhirString>),
    /// `addressContactPoint`
    ContactPoint(Box<ContactPoint>),
    /// `addressExtendedContactDetail`
    ExtendedContactDetail(Box<ExtendedContactDetail>),
}

/// The FHIR `VirtualServiceDetail` metadata type: how to connect to a virtual meeting
/// or service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualServiceDetail {
    id: Option<FhirString>,
    extension: Vec<Extension>,
    channel_type: Option<Coding>,
    address: Option<VirtualServiceAddress>,
    additional_info: Vec<Primitive<Url>>,
    max_participants: Option<Primitive<PositiveInt>>,
    session_key: Option<Primitive<FhirString>>,
}
