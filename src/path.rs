use serde::{Deserialize, Serialize};
use crate::components::ObjectOrReference;
use crate::spec::{Schema};
use crate::server::Server;
use crate::operation::{Operation};
use actix_web::client::{ClientRequest, ClientRequestBuilder};
use actix_web::http::{Method};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
enum ParameterStyle {
    Form,
    Simple,
}


// FIXME: Verify against OpenAPI 3.0
/// Describes a single operation parameter.
/// A unique parameter is defined by a combination of a
/// [name](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterName)
/// and [location](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterIn).
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Parameter {
    /// The name of the parameter.
    name: String,
    /// values depend on parameter type
    /// may be `header`, `query`, 'path`, `formData`
    #[serde(rename = "in")]
    location: String,
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
    /// Describes how the parameter value will be serialized depending on the type of the parameter
    /// value. Default values (based on value of in): for `query` - `form`; for `path` - `simple`; for
    /// `header` - `simple`; for cookie - `form`.
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<ParameterStyle>,
}

/// Describes the operations available on a single path.
/// A Path Item MAY be empty, due to
/// [ACL constraints](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#securityFiltering).
/// The path itself is still exposed to the documentation viewer but they will
/// not know which operations and parameters are available.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct PathItem {
    /// Allows for an external definition of this path item. The referenced structure MUST be
    /// in the format of a
    /// [Path Item Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#pathItemObject).
    /// If there are conflicts between the referenced definition and this Path Item's definition,
    /// the behavior is undefined.
    // FIXME: Should this ref be moved to an enum?
    #[serde(skip_serializing_if = "Option::is_none", rename = "$ref")]
    pub reference: Option<String>,

    /// An optional, string summary, intended to apply to all operations in this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// An optional, string description, intended to apply to all operations in this path.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    /// A definition of a PUT operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    /// A definition of a POST operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    /// A definition of a DELETE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    /// A definition of a OPTIONS operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    /// A definition of a HEAD operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    /// A definition of a PATCH operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    /// A definition of a TRACE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,

    /// An alternative `server` array to service all operations in this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,

    /// A list of parameters that are applicable for all the operations described under this
    /// path. These parameters can be overridden at the operation level, but cannot be removed
    /// there. The list MUST NOT include duplicated parameters. A unique parameter is defined by
    /// a combination of a
    /// [name](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterName)
    /// and
    /// [location](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterIn).
    /// The list can use the
    /// [Reference Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#referenceObject)
    /// to link to parameters that are defined at the
    /// [OpenAPI Object's components/parameters](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#componentsParameters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ObjectOrReference<Parameter>>>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

impl PathItem {
    fn apply_operation(&self, method: Method, servers: Vec<Server>) -> Vec<ClientRequestBuilder> {
        let mut builders: Vec<ClientRequestBuilder> = Vec::new();

        for server in servers.iter() {
            let mut builder = ClientRequest::build();
            // pass builder to Operation to update uri
            builder.uri(&server.url);
            //
            builder.method(method.clone());
            builders.push(builder);
        }

        builders
    }
    // parameter is Vec<Server> which is then iterated and applied to every path
    pub fn to_client_request(&self, servers: Vec<Server>) -> Vec<ClientRequestBuilder> {
        let mut builders: Vec<ClientRequestBuilder> = Vec::new();
        let lst_methods:Vec<Method> = Vec::new();
        
        if self.get.is_some() {
            builders.append(&self.apply_operation(Method::GET, servers));
        }

        if self.put.is_some() {
            lst_methods.push(Method::PUT);
        }

        if self.post.is_some() {
            lst_methods.push(Method::POST);
        }

        if self.delete.is_some() {
            lst_methods.push(Method::DELETE);
        }

        if self.options.is_some() {
            lst_methods.push(Method::OPTIONS);
        }

        if self.head.is_some() {
            lst_methods.push(Method::HEAD);
        }

        if self.patch.is_some() {
            lst_methods.push(Method::PATCH);
        }

        if self.trace.is_some() {
            lst_methods.push(Method::TRACE);
        }

        for method in lst_methods {
            for server in servers.iter() {
                let mut builder = ClientRequest::build();
                // pass builder to Operation to update uri
                builder.uri(&server.url);
                //
                builder.method(method.clone());
                builders.push(builder);
            }
        }

        builders
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation::Operation;
    use std::collections::BTreeMap;
    use crate::spec::{Response};
    
    #[test]
    fn test_to_client_request_default() {
        let mut servers:Vec<Server> = Vec::new();
        servers.push(
            Server{
                url: "http://localhost:8000/v1".to_string(),
                description: None,
                variables: None,
            });

        let paths = PathItem {
            reference: None,
            summary: None,
            description: None,
            get: None,
            put: None,
            post: None,
            delete: None,
            options: None,
            head: None,
            patch: None,
            trace: None,
            servers: None,
            parameters: None,
        };

        for mut path in paths.to_client_request(servers) {
            match path.get_method() {
                &Method::GET => assert!(true),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_to_client_request_multi() {
        let respns: BTreeMap<String, Response> = BTreeMap::new();

        let mut servers:Vec<Server> = Vec::new();
        servers.push(
            Server{
                url: "http://localhost:8000/v1".to_string(),
                description: None,
                variables: None,
            });

        let get_oper = Operation {
            tags: None,
            summary: None,
            description: None,
            external_docs: None,
            operation_id: None,
            parameters: None,
            request_body: None,
            responses: respns.clone(),
            callbacks: None,
            deprecated: Some(false),
            servers: None,
        };
        let post_oper = Operation {
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

        let paths = PathItem {
            reference: None,
            summary: None,
            description: None,
            get: Some(get_oper),
            put: None,
            post: Some(post_oper),
            delete: None,
            options: None,
            head: None,
            patch: None,
            trace: None,
            servers: None,
            parameters: None,
        };

        let clients = paths.to_client_request(servers);
        assert_eq!(clients.len(), 2);

        for mut client in clients {
            match client.get_method() {
                &Method::GET => assert!(true),
                &Method::POST => assert!(true),
                _ => assert!(false),
            }

            assert_eq!(client.finish().unwrap().uri().path(), "/v1");
        }
    }
}