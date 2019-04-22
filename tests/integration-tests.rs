extern crate actix_web_openapi;

use actix_web::client::{ClientRequest};

#[test]
fn test_server(){
    match actix_web_openapi::from_path("./data/v3.0/petstore.yaml") {
        Ok(spec) => {
            let servers = spec.servers.unwrap();

            for server in servers.iter() {
                let builder = ClientRequest::build();

                match server.to_client_request(builder).finish() {
                    Ok(client) => {
                        assert_eq!(client.uri().scheme_str(), Some("http"));
                        assert_eq!(client.uri().host(), Some("petstore.swagger.io"));
                        assert_eq!(client.uri().port_u16(), None);
                        assert_eq!(client.uri().path(), "/v1");
                    },
                    Err(_err) => assert!(false),
                } 
            }
        },
        Err(_err) => assert!(false),
    }
}

#[test]
fn test_spec(){
    match actix_web_openapi::from_path("./data/v3.0/petstore.yaml") {
        Ok(spec) => {
            match spec.to_client_request() {
                Ok(clients) => {
                    for client in clients.iter() {
                        assert_eq!(client.uri().scheme_str(), Some("http"));
                        assert_eq!(client.uri().host(), Some("petstore.swagger.io"));
                        assert_eq!(client.uri().port_u16(), None);
                        assert_eq!(client.uri().path(), "/v1");
                    }
                },
                Err(_err) => assert!(false),
            }
        },
        Err(_err) => assert!(false),
    }
}