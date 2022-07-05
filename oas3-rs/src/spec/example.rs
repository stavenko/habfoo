use serde::{Deserialize, Serialize};

use super::{FromRef, Ref, RefError, RefType, Spec, read_from_file};

/// Multi-purpose example objects.
///
/// Will be validated against schema when used in conformance testing.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/HEAD/versions/3.1.0.md#exampleObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Example {
    /// Short description for the example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Long description for the example.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    // FIXME: Implement (merge with externalValue as enum)
    /// Embedded literal example. The `value` field and `externalValue` field are mutually
    /// exclusive. To represent examples of media types that cannot naturally represented
    /// in JSON or YAML, use a string value to contain the example, escaping where necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    // FIXME: Implement (merge with value as enum)
    // /// A URL that points to the literal example. This provides the capability to reference
    // /// examples that cannot easily be included in JSON or YAML documents. The `value` field
    // /// and `externalValue` field are mutually exclusive.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub externalValue: Option<String>,

    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/HEAD/versions/3.1.0.md#specificationExtensions}
}

impl Example {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self.value {
            Some(ref val) => serde_json::to_string(val).unwrap().as_bytes().to_owned(),
            None => vec![],
        }
    }
}

impl FromRef for Example {
    fn from_ref(spec: &Spec, path: &str) -> Result<Self, RefError> {
        let refpath = path.parse::<Ref>()?;

        match refpath {
            Ref::Path(path) => read_from_file(spec, path),
            Ref::InFile { source, kind, name } => match kind {
                RefType::Example => spec
                    .components
                    .as_ref()
                    .and_then(|cs| cs.examples.get(&name))
                    .ok_or_else(|| RefError::Unresolvable(path.to_owned()))
                    .and_then(|oor| oor.resolve(&spec)),

                typ => Err(RefError::MismatchedType(typ, RefType::Example)),
            },
        }
    }
}
