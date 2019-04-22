use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use actix_web::client::{ClientRequest, ClientRequestBuilder};

/// An object representing a Server.
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#serverObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Server {
    /// A URL to the target host. This URL supports Server Variables and MAY be relative, to
    /// indicate that the host location is relative to the location where the OpenAPI document
    /// is being served. Variable substitutions will be made when a variable is named
    /// in {brackets}.
    pub url: String,
    /// An optional string describing the host designated by the URL. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A map between a variable name and its value. The value is used for substitution in
    /// the server's URL template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<BTreeMap<String, ServerVariable>>,
}

/// An object representing a Server Variable for server URL template substitution.
///
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#serverVariableObject]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ServerVariable {
    /// The default value to use for substitution, and to send, if an alternate value is not
    /// supplied. Unlike the Schema Object's default, this value MUST be provided by the consumer.
    pub default: String,
    /// An enumeration of string values to be used if the substitution options are from a limited
    /// set.
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub substitutions_enum: Option<Vec<String>>,
    /// An optional description for the server variable. [CommonMark] syntax MAY be used for rich
    /// text representation.
    ///
    /// [CommonMark]: https://spec.commonmark.org/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Server {
    pub fn to_client_request(&self, mut builder: ClientRequestBuilder) -> ClientRequestBuilder {
        builder.uri(&self.url);

        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_to_client_request() {
        let mut builder = ClientRequest::build();
        let server = Server {
            url: "http://localhost:8000/v1".to_string(),
            description: None,
            variables: None,
        };

        match server.to_client_request(builder).finish() {
            Ok(client) => {
                assert_eq!(client.uri().scheme_str(), Some("http"));
                assert_eq!(client.uri().host(), Some("localhost"));
                assert_eq!(client.uri().port_u16(), Some(8000));
                assert_eq!(client.uri().path(), "/v1");
            },
            Err(_err) => assert!(false),
        }        
    }
}