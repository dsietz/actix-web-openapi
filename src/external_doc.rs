use serde::{Deserialize, Serialize};

/// Allows referencing an external resource for extended documentation.
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#externalDocumentationObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ExternalDoc {
    /// The URL for the target documentation. Value MUST be in the format of a URL.
    // FIXME: Use `url::Url` instead of `String`
    // #[serde(with = "url_serde")]
    // pub url: url::Url,
    pub url: String,

    /// A short description of the target documentation.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}