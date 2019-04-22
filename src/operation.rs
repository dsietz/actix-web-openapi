use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::spec::{Callback, RequestBody, Response};
use crate::server::Server;
use crate::components::ObjectOrReference;
use crate::path::Parameter;
use crate::external_doc::ExternalDoc;
use actix_web::client::{ClientRequestBuilder};

/// Describes a single API operation on a path.
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#operationObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
// #[serde(rename_all = "lowercase")]
pub struct Operation {
    /// A list of tags for API documentation control. Tags can be used for logical grouping of
    /// operations by resources or any other qualifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// A short summary of what the operation does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<ExternalDoc>,
    /// Unique string used to identify the operation. The id MUST be unique among all operations
    /// described in the API. Tools and libraries MAY use the operationId to uniquely identify an
    /// operation, therefore, it is RECOMMENDED to follow common programming naming conventions.
    #[serde(skip_serializing_if = "Option::is_none", rename = "operationId")]
    pub operation_id: Option<String>,

    /// A list of parameters that are applicable for this operation. If a parameter is already
    /// defined at the
    /// [Path Item](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#pathItemParameters),
    /// the new definition will override it but can never remove it. The list MUST NOT
    /// include duplicated parameters. A unique parameter is defined by a combination of a
    /// [name](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterName)
    /// and
    /// [location](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterIn).
    /// The list can use the
    /// [Reference Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#referenceObject)
    /// to link to parameters that are defined at the
    /// [OpenAPI Object's components/parameters](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#componentsParameters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ObjectOrReference<Parameter>>>,

    /// The request body applicable for this operation. The requestBody is only supported in HTTP methods where the HTTP 1.1 specification RFC7231 has explicitly defined semantics for request bodies. In other cases where the HTTP spec is vague, requestBody SHALL be ignored by consumers.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestBody")]
    pub request_body: Option<ObjectOrReference<RequestBody>>,

    /// The list of possible responses as they are returned from executing this operation.
    ///
    /// A container for the expected responses of an operation. The container maps a HTTP
    /// response code to the expected response.
    ///
    /// The documentation is not necessarily expected to cover all possible HTTP response codes
    /// because they may not be known in advance. However, documentation is expected to cover
    /// a successful operation response and any known errors.
    ///
    /// The `default` MAY be used as a default response object for all HTTP codes that are not
    /// covered individually by the specification.
    ///
    /// The `Responses Object` MUST contain at least one response code, and it SHOULD be the
    /// response for a successful operation call.
    ///
    /// See [link]
    /// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#responsesObject]
    pub responses: BTreeMap<String, Response>,

    /// A map of possible out-of band callbacks related to the parent operation. The key is
    /// a unique identifier for the Callback Object. Each value in the map is a
    /// [Callback Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#callbackObject)
    /// that describes a request that may be initiated by the API provider and the
    /// expected responses. The key value used to identify the callback object is
    /// an expression, evaluated at runtime, that identifies a URL to use for the
    /// callback operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<BTreeMap<String, Callback>>,

    /// Declares this operation to be deprecated. Consumers SHOULD refrain from usage
    /// of the declared operation. Default value is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    // FIXME: Implement
    // /// A declaration of which security mechanisms can be used for this operation. The list of
    // /// values includes alternative security requirement objects that can be used. Only one
    // /// of the security requirement objects need to be satisfied to authorize a request.
    // /// This definition overrides any declared top-level
    // /// [`security`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#oasSecurity).
    // /// To remove a top-level security declaration, an empty array can be used.
    // pub security: Option<SecurityRequirement>,
    /// An alternative `server` array to service this operation. If an alternative `server`
    /// object is specified at the Path Item Object or Root level, it will be overridden by
    /// this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
}

impl Operation {
    pub fn to_client_request(&self, mut builder: ClientRequestBuilder) -> ClientRequestBuilder {
        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::client::{ClientRequest};
    
    #[test]
    fn test_to_client_request() {
        let mut builder = ClientRequest::build();
        let respns: BTreeMap<String, Response> = BTreeMap::new();

        let operation = Operation {
            tags: None,
            summary: None,
            description: None,
            external_docs: None,
            operation_id: None,
            parameters: None,
            request_body: None,
            responses: respns,
            callbacks: None,
            deprecated: Some(false),
            servers: None,
        };

        match operation.to_client_request(builder).finish() {
            Ok(client) => {
                assert!(true);
            },
            Err(_err) => assert!(false),
        }    
    }
}