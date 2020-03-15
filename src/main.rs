#![feature(proc_macro_hygiene, decl_macro)] // language features needed by Rocket

// Import the rocket macros
#[macro_use]
extern crate rocket;

// Import OpenAPI macros
#[macro_use]
extern crate rocket_okapi;

use rocket_contrib::json::Json;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{JsonSchema, OpenApiError, Result};
use serde::*;

/// Host information structure returned at /hostinfo
#[derive(Serialize, JsonSchema, Debug)]
struct HostInfo {
    hostname: String,
    pid: u32,
    uptime: u64,
}

/// Create route / that returns "Hello, world!"
#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Create route /hostinfo that returns information about the host serving
/// this page.
#[openapi]
#[get("/hostinfo")]
fn hostinfo() -> Result<Json<HostInfo>> {
    match gethostname::gethostname().into_string() {
        Ok(hostname) => Ok(Json(HostInfo {
            hostname: hostname,
            pid: std::process::id(),
            uptime: psutil::host::uptime().unwrap().as_secs(),
        })),
        Err(_) => Err(OpenApiError::new(format!(
            "hostname does not parse as UTF-8"
        ))),
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes_with_openapi![index, hostinfo])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: Some("../openapi.json".to_owned()),
                urls: None,
            }),
        )
        .launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::*;

    #[test]
    fn test_index() {
        let rkt = rocket::ignite().mount("/", routes![index]);
        let client = Client::new(rkt).expect("valid rocket");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn test_hostinfo() {
        let rkt = rocket::ignite().mount("/", routes![hostinfo]);
        let client = Client::new(rkt).expect("valid rocket");
        let response = client.get("/hostinfo").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
