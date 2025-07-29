// server/src/main.rs
use axum::{routing::get, Json, Router};
use shared::service::build_hello;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/hello", get(hello_online));
    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_online() -> Json<shared::models::HelloResponse> {
    Json(build_hello("cloud"))
}