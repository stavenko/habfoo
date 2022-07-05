use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{FromRef, Header, Link, MediaType, ObjectOrReference, Ref, RefError, RefType, Spec, read_from_file};

/// Describes a single response from an API Operation, including design-time, static `links`
/// to operations based on the response.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/HEAD/versions/3.1.0.md#responseObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Response {
    /// A short description of the response.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    pub description: Option<String>,

    /// Maps a header name to its definition.
    /// [RFC7230](https://tools.ietf.org/html/rfc7230#page-22) states header names are case
    /// insensitive. If a response header is defined with the name `"Content-Type"`, it SHALL
    /// be ignored.
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, ObjectOrReference<Header>>,

    /// A map containing descriptions of potential response payloads. The key is a media type
    /// or [media type range](https://tools.ietf.org/html/rfc7231#appendix-D) and the value
    /// describes it. For responses that match multiple keys, only the most specific key is
    /// applicable. e.g. text/plain overrides text/*
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub content: BTreeMap<String, MediaType>,

    /// A map of operations links that can be followed from the response. The key of the map
    /// is a short name for the link, following the naming constraints of the names for
    /// [Component Objects](https://github.com/OAI/OpenAPI-Specification/blob/HEAD/versions/3.1.0.md#componentsObject).
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, ObjectOrReference<Link>>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/HEAD/versions/3.1.0.md#specificationExtensions}
}

impl FromRef for Response {
    fn from_ref(spec: &Spec, path: &str) -> Result<Self, RefError> {
        let refpath = path.parse::<Ref>()?;

        match refpath {
            Ref::Path(path) => read_from_file(spec, path),
            Ref::InFile { source, kind, name } => match kind {
                RefType::Response => spec
                    .components
                    .as_ref()
                    .and_then(|cs| cs.responses.get(&name))
                    .ok_or_else(|| RefError::Unresolvable(path.to_owned()))
                    .and_then(|oor| oor.resolve(&spec)),

                typ => Err(RefError::MismatchedType(typ, RefType::Response)),
            },
        }
    }
}
