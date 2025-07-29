// shared/src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloResponse {
    pub source: String,
    pub message: String,
}
