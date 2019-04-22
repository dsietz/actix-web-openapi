//! Schema specification for [OpenAPI 3.0.0](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.0.md)

use semver;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use url;
use url_serde;
use actix_web::client::{ClientRequest};

use crate::server::{Server};
use crate::path::{PathItem};
use crate::external_doc::{ExternalDoc};
use crate::components::{Components, ObjectOrReference};
use crate::{Error, Result, MINIMUM_OPENAPI30_VERSION};
    

impl Spec {
    pub fn validate_version(&self) -> Result<semver::Version> {
        let spec_version = &self.openapi;
        let sem_ver = semver::Version::parse(spec_version)?;
        let required_version = semver::VersionReq::parse(MINIMUM_OPENAPI30_VERSION).unwrap();
        if required_version.matches(&sem_ver) {
            Ok(sem_ver)
        } else {
            Err(Error::UnsupportedSpecFileVersion(sem_ver))?
        }
    }

    pub fn to_client_request(&self) -> Result<Vec<ClientRequest>> {
        let mut client_requests: Vec<ClientRequest> = Vec::new();

        //get the servers
        let servers = &self.servers.clone().unwrap();

        for server in servers.iter() {
            //stub out calling path.to_client_request() -> Vec<ClientRequestBuilder>
            let mut builder = ClientRequest::build();

            match server.to_client_request(builder).finish() {
                Ok(client) => {
                        //apply Path configurations
                        for path in self.paths.iter() {
                            println!("PATH: {:?}", path);
                        }

                        client_requests.push(client)
                    },
                Err(_err) => panic!("Could not convert server to client request! {:?}", server),
            } 
        }
        
        Ok(client_requests)
    }
}

/// top level document
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Spec {
    /// This string MUST be the [semantic version number](https://semver.org/spec/v2.0.0.html)
    /// of the
    /// [OpenAPI Specification version](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#versions)
    /// that the OpenAPI document uses. The `openapi` field SHOULD be used by tooling
    /// specifications and clients to interpret the OpenAPI document. This is not related to
    /// the API
    /// [`info.version`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#infoVersion)
    /// string.
    pub openapi: String,
    /// Provides metadata about the API. The metadata MAY be used by tooling as required.
    pub info: Info,
    /// An array of Server Objects, which provide connectivity information to a target server.
    /// If the `servers` property is not provided, or is an empty array, the default value would
    /// be a
    /// [Server Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#serverObject)
    /// with a
    /// [url](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#serverUrl)
    /// value of `/`.
    // FIXME: Provide a default value as specified in documentation instead of `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,

    /// Holds the relative paths to the individual endpoints and their operations. The path is
    /// appended to the URL from the
    /// [`Server Object`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#serverObject)
    /// in order to construct the full URL. The Paths MAY be empty, due to
    /// [ACL constraints](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#securityFiltering).
    pub paths: BTreeMap<String, PathItem>,

    /// An element to hold various schemas for the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,

    // FIXME: Implement
    // /// A declaration of which security mechanisms can be used across the API.
    // /// The list of  values includes alternative security requirement objects that can be used.
    // /// Only one of the security requirement objects need to be satisfied to authorize a request.
    // /// Individual operations can override this definition.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub security: Option<SecurityRequirement>,
    /// A list of tags used by the specification with additional metadata.
    ///The order of the tags can be used to reflect on their order by the parsing tools.
    /// Not all tags that are used by the
    /// [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#operationObject)
    /// must be declared. The tags that are not declared MAY be organized randomly or
    /// based on the tools' logic. Each tag name in the list MUST be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    /// Additional external documentation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<ExternalDoc>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

/// General information about the API.
///
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#infoObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
// #[serde(rename_all = "lowercase")]
pub struct Info {
    /// The title of the application.
    pub title: String,
    /// A short description of the application. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API. MUST be in the format of a URL.
    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<Url>,
    /// REQUIRED. The version of the OpenAPI document (which is distinct from the [OpenAPI Specification version](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#oasVersion)
    /// or the API implementation version).
    pub version: String,
    /// The contact information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
}

/// Wraper around `url::Url` to fix serde issue
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Url(#[serde(with = "url_serde")] url::Url);

/// Contact information for the exposed API.
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#contactObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    // TODO: Make sure the email is a valid email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions
}

/// License information for the exposed API.
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#licenseObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct License {
    /// The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

// FIXME: Verify against OpenAPI 3.0
/// The Schema Object allows the definition of input and output data types.
/// These types can be objects, but also primitives and arrays.
/// This object is an extended subset of the
/// [JSON Schema Specification Wright Draft 00](http://json-schema.org/).
/// For more information about the properties, see
/// [JSON Schema Core](https://tools.ietf.org/html/draft-wright-json-schema-00) and
/// [JSON Schema Validation](https://tools.ietf.org/html/draft-wright-json-schema-validation-00).
/// Unless stated otherwise, the property definitions follow the JSON Schema.
///
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#schemaObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Schema {
    /// [JSON reference](https://tools.ietf.org/html/draft-pbryan-zyp-json-ref-03)
    /// path to another defintion
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$ref")]
    pub ref_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub schema_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, Schema>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "readOnly")]
    pub read_only: Option<bool>,

    /// Value can be boolean or object. Inline or referenced schema MUST be of a
    /// [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#schemaObject)
    /// and not a standard JSON Schema.
    /// See [link]
    /// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#properties]
    // FIXME: Why can this be a "boolean" (as per the spec)? It doesn't make sense. Here it's not.
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "additionalProperties"
    )]
    pub additional_properties: Option<ObjectOrReference<Box<Schema>>>,

    /// A free-form property to include an example of an instance for this schema.
    /// To represent examples that cannot be naturally represented in JSON or YAML,
    /// a string value can be used to contain the example with escaping where necessary.
    /// NOTE: According to [spec], _Primitive data types in the OAS are based on the
    ///       types supported by the JSON Schema Specification Wright Draft 00._
    ///       This suggest using
    ///       [`serde_json::Value`](https://docs.serde.rs/serde_json/value/enum.Value.html). [spec][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#data-types]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::value::Value>,

    // The following properties are taken directly from the JSON Schema definition and
    // follow the same specifications:
    // title
    // multipleOf
    // maximum
    // exclusiveMaximum
    // minimum
    // exclusiveMinimum
    // maxLength
    // minLength
    // pattern (This string SHOULD be a valid regular expression, according to the ECMA 262 regular expression dialect)
    // maxItems
    // minItems
    // uniqueItems
    // maxProperties
    // minProperties
    // required
    // enum

    // The following properties are taken from the JSON Schema definition but their
    // definitions were adjusted to the OpenAPI Specification.
    // - type - Value MUST be a string. Multiple types via an array are not supported.
    // - allOf - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - oneOf - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - anyOf - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - not - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - items - Value MUST be an object and not an array. Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema. `items` MUST be present if the `type` is `array`.
    // - properties - Property definitions MUST be a [Schema Object](#schemaObject) and not a standard JSON Schema (inline or referenced).
    // - additionalProperties - Value can be boolean or object. Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - description - [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    // - format - See [Data Type Formats](#dataTypeFormat) for further details. While relying on JSON Schema's defined formats, the OAS offers a few additional predefined formats.
    // - default - The default value represents what would be assumed by the consumer of the input as the value of the schema if one is not provided. Unlike JSON Schema, the value MUST conform to the defined type for the Schema Object defined at the same level. For example, if `type` is `string`, then `default` can be `"foo"` but cannot be `1`.
    /// The default value represents what would be assumed by the consumer of the input as the value
    /// of the schema if one is not provided. Unlike JSON Schema, the value MUST conform to the
    /// defined type for the Schema Object defined at the same level. For example, if type is
    /// `string`, then `default` can be `"foo"` but cannot be `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<serde_json::Value>,

    /// Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    /// JSON Schema.
    #[serde(rename = "allOf", skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<ObjectOrReference<Schema>>>,
}

/// Describes a single response from an API Operation, including design-time, static `links`
/// to operations based on the response.
///
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#responseObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Response {
    /// A short description of the response.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    pub description: Option<String>,

    /// Maps a header name to its definition.
    /// [RFC7230](https://tools.ietf.org/html/rfc7230#page-22) states header names are case
    /// insensitive. If a response header is defined with the name `"Content-Type"`, it SHALL
    /// be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, ObjectOrReference<Header>>>,

    /// A map containing descriptions of potential response payloads. The key is a media type
    /// or [media type range](https://tools.ietf.org/html/rfc7231#appendix-D) and the value
    /// describes it. For responses that match multiple keys, only the most specific key is
    /// applicable. e.g. text/plain overrides text/*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<BTreeMap<String, MediaType>>,

    /// A map of operations links that can be followed from the response. The key of the map
    /// is a short name for the link, following the naming constraints of the names for
    /// [Component Objects](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#componentsObject).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<BTreeMap<String, ObjectOrReference<Link>>>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

/// The Header Object follows the structure of the
/// [Parameter Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterObject)
/// with the following changes:
/// 1. `name` MUST NOT be specified, it is given in the corresponding `headers` map.
/// 1. `in` MUST NOT be specified, it is implicitly in `header`.
/// 1. All traits that are affected by the location MUST be applicable to a location of
///    `header` (for example, [`style`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterStyle)).
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#headerObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Header {
    // FIXME: Is the third change properly implemented?
    // FIXME: Merge `ObjectOrReference<Header>::Reference` and `ParameterOrRef::Reference`
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "uniqueItems")]
    unique_items: Option<bool>,
    /// string, number, boolean, integer, array, file ( only for formData )
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    param_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    /// A brief description of the parameter. This could contain examples
    /// of use.  GitHub Flavored Markdown is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    // collectionFormat: ???
    // default: ???
    // maximum ?
    // exclusiveMaximum ??
    // minimum ??
    // exclusiveMinimum ??
    // maxLength ??
    // minLength ??
    // pattern ??
    // maxItems ??
    // minItems ??
    // enum ??
    // multipleOf ??
    // allowEmptyValue ( for query / body params )
}

/// Describes a single request body.
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#requestBodyObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct RequestBody {
    /// A brief description of the request body. This could contain examples of use.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The content of the request body. The key is a media type or
    /// [media type range](https://tools.ietf.org/html/rfc7231#appendix-D) and the
    /// value describes it. For requests that match multiple keys, only the most specific key
    /// is applicable. e.g. text/plain overrides text/*
    pub content: BTreeMap<String, MediaType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// The Link object represents a possible design-time link for a response.
///
/// The presence of a link does not guarantee the caller's ability to successfully invoke it,
/// rather it provides a known relationship and traversal mechanism between responses and
/// other operations.
///
/// Unlike _dynamic_ links (i.e. links provided *in* the response payload), the OAS linking
/// mechanism does not require link information in the runtime response.
///
/// For computing links, and providing instructions to execute them, a
/// [runtime expression](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#runtimeExpression)
/// is used for accessing values in an operation and using them as parameters while invoking
/// the linked operation.
///
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#linkObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Link {
    /// A relative or absolute reference to an OAS operation. This field is mutually exclusive
    /// of the `operationId` field, and MUST point to an
    /// [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#operationObject).
    /// Relative `operationRef` values MAY be used to locate an existing
    /// [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#operationObject)
    /// in the OpenAPI definition.
    Ref {
        #[serde(rename = "operationRef")]
        operation_ref: String,

        // FIXME: Implement
        // /// A map representing parameters to pass to an operation as specified with `operationId`
        // /// or identified via `operationRef`. The key is the parameter name to be used, whereas
        // /// the value can be a constant or an expression to be evaluated and passed to the
        // /// linked operation. The parameter name can be qualified using the
        // /// [parameter location](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterIn)
        // /// `[{in}.]{name}` for operations that use the same parameter name in different
        // /// locations (e.g. path.id).
        // parameters: BTreeMap<String, Any | {expression}>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<BTreeMap<String, String>>,

        // FIXME: Implement
        // /// A literal value or
        // /// [{expression}](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#runtimeExpression)
        // /// to use as a request body when calling the target operation.
        // #[serde(rename = "requestBody")]
        // request_body: Any | {expression}
        /// A description of the link. [CommonMark syntax](http://spec.commonmark.org/) MAY be
        /// used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        /// A server object to be used by the target operation.
        #[serde(skip_serializing_if = "Option::is_none")]
        server: Option<Server>,
        // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtension
    },
    /// The name of an _existing_, resolvable OAS operation, as defined with a unique
    /// `operationId`. This field is mutually exclusive of the `operationRef` field.
    Id {
        #[serde(rename = "operationId")]
        operation_id: String,

        // FIXME: Implement
        // /// A map representing parameters to pass to an operation as specified with `operationId`
        // /// or identified via `operationRef`. The key is the parameter name to be used, whereas
        // /// the value can be a constant or an expression to be evaluated and passed to the
        // /// linked operation. The parameter name can be qualified using the
        // /// [parameter location](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterIn)
        // /// `[{in}.]{name}` for operations that use the same parameter name in different
        // /// locations (e.g. path.id).
        // parameters: BTreeMap<String, Any | {expression}>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<BTreeMap<String, String>>,

        // FIXME: Implement
        // /// A literal value or
        // /// [{expression}](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#runtimeExpression)
        // /// to use as a request body when calling the target operation.
        // #[serde(rename = "requestBody")]
        // request_body: Any | {expression}
        /// A description of the link. [CommonMark syntax](http://spec.commonmark.org/) MAY be
        /// used for rich text representation.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        /// A server object to be used by the target operation.
        #[serde(skip_serializing_if = "Option::is_none")]
        server: Option<Server>,
        // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtension
    },
}

/// Each Media Type Object provides schema and examples for the media type identified by its key.
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#media-type-object]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct MediaType {
    /// The schema defining the type used for the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ObjectOrReference<Schema>>,

    /// Example of the media type.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub examples: Option<MediaTypeExample>,

    /// A map between a property name and its encoding information. The key, being the
    /// property name, MUST exist in the schema as a property. The encoding object SHALL
    /// only apply to `requestBody` objects when the media type is `multipart`
    /// or `application/x-www-form-urlencoded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<BTreeMap<String, Encoding>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum MediaTypeExample {
    /// Example of the media type. The example object SHOULD be in the correct format as
    /// specified by the media type. The `example` field is mutually exclusive of the
    /// `examples` field. Furthermore, if referencing a `schema` which contains an example,
    /// the `example` value SHALL override the example provided by the schema.
    Example { example: serde_json::Value },
    /// Examples of the media type. Each example object SHOULD match the media type and
    /// specified schema if present. The `examples` field is mutually exclusive of
    /// the `example` field. Furthermore, if referencing a `schema` which contains an
    /// example, the `examples` value SHALL override the example provided by the schema.
    Examples {
        examples: BTreeMap<String, ObjectOrReference<Example>>,
    },
}

/// A single encoding definition applied to a single schema property.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Encoding {
    /// The Content-Type for encoding a specific property. Default value depends on the
    /// property type: for `string` with `format` being `binary` – `application/octet-stream`;
    /// for other primitive types – `text/plain`; for `object` - `application/json`;
    /// for `array` – the default is defined based on the inner type. The value can be a
    /// specific media type (e.g. `application/json`), a wildcard media type
    /// (e.g. `image/*`), or a comma-separated list of the two types.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentType")]
    pub content_type: Option<String>,

    /// A map allowing additional information to be provided as headers, for example
    /// `Content-Disposition`.  `Content-Type` is described separately and SHALL be
    /// ignored in this section. This property SHALL be ignored if the request body
    /// media type is not a `multipart`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, ObjectOrReference<Header>>>,

    /// Describes how a specific property value will be serialized depending on its type.
    /// See [Parameter Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterObject)
    /// for details on the
    /// [`style`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterStyle)
    /// property. The behavior follows the same values as `query` parameters, including
    /// default values. This property SHALL be ignored if the request body media type
    /// is not `application/x-www-form-urlencoded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,

    /// When this is true, property values of type `array` or `object` generate
    /// separate parameters for each value of the array, or key-value-pair of the map.
    /// For other types of properties this property has no effect. When
    /// [`style`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#encodingStyle)
    /// is `form`, the default value is `true`. For all other styles, the default value
    /// is `false`. This property SHALL be ignored if the request body media type is
    /// not `application/x-www-form-urlencoded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,

    /// Determines whether the parameter value SHOULD allow reserved characters, as defined
    /// by [RFC3986](https://tools.ietf.org/html/rfc3986#section-2.2) `:/?#[]@!$&'()*+,;=`
    /// to be included without percent-encoding. The default value is `false`. This
    /// property SHALL be ignored if the request body media type is
    /// not `application/x-www-form-urlencoded`.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowReserved")]
    pub allow_reserved: Option<bool>,
}

///
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#exampleObject]
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

    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

/// Defines a security scheme that can be used by the operations. Supported schemes are
/// HTTP authentication, an API key (either as a header or as a query parameter),
///OAuth2's common flows (implicit, password, application and access code) as defined
/// in [RFC6749](https://tools.ietf.org/html/rfc6749), and
/// [OpenID Connect Discovery](https://tools.ietf.org/html/draft-ietf-oauth-discovery-06).
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#securitySchemeObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum SecurityScheme {
    #[serde(rename = "apiKey")]
    ApiKey {
        name: String,
        #[serde(rename = "in")]
        location: String,
    },
    #[serde(rename = "http")]
    Http {
        scheme: String,
        #[serde(rename = "bearerFormat")]
        bearer_format: String,
    },
    // FIXME: Implement
    // #[serde(rename = "oauth2")]
    // Oauth2 {
    //     flow: String,
    //     #[serde(rename = "authorizationUrl")]
    //     authorization_url: String,
    //     #[serde(rename = "tokenUrl")]
    //     #[serde(skip_serializing_if = "Option::is_none")]
    //     token_url: Option<String>,
    //     scopes: BTreeMap<String, String>,
    // },
    #[serde(rename = "openIdConnect")]
    OpenIdConnect {
        #[serde(rename = "openIdConnectUrl")]
        open_id_connect_url: String,
    },
}

// TODO: Implement
/// A map of possible out-of band callbacks related to the parent operation. Each value in
/// the map is a Path Item Object that describes a set of requests that may be initiated by
/// the API provider and the expected responses. The key value used to identify the callback
/// object is an expression, evaluated at runtime, that identifies a URL to use for the
/// callback operation.
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#callbackObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Callback(
    /// A Path Item Object used to define a callback request and expected responses.
    serde_json::Value, // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
);

// FIXME: Implement
// /// Allows configuration of the supported OAuth Flows.
// /// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#oauthFlowsObject
// #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
// pub struct OAuthFlows {
// }

/// Adds metadata to a single tag that is used by the
/// [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#operationObject).
/// It is not mandatory to have a Tag Object per tag defined in the Operation Object instances.
///
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#tagObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Tag {
    /// The name of the tag.
    pub name: String,

    /// A short description for the tag.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    // /// Additional external documentation for this tag.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub external_docs: Option<Vec<ExternalDoc>>,

    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

