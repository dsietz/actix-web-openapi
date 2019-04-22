# Actix Web OpenAPI
![Version](https://img.shields.io/badge/version-0.0.1-blue.svg)
![MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

---

## Usage
```
extern crate actix_web_openapi;

fn main() {
    match actix_web_openapi::from_path("./data/v3.0/petstore.yaml") {
        Ok(spec) => {
            let servers = spec.servers.unwrap();

            for server in servers.iter() {
                match server.to_client_request().finish() {
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
```

## License
This soltuion uses open source products and is constrained to the inherited license agreements.

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
