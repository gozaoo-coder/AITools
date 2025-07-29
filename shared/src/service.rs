// shared/src/service.rs
use crate::models::HelloResponse;

pub fn build_hello(source: &str) -> HelloResponse {
    HelloResponse {
        source: source.to_string(),
        message: format!("Hello from {}!", source),
    }
}