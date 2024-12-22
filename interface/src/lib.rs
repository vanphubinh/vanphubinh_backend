use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};

pub fn new() -> Router {
  Router::new()
    .route("/", get(root))
    .route("/complex", get(complex))
}
async fn root() -> &'static str {
  "Hello, World!"
}

async fn complex() -> impl IntoResponse {
  (
    StatusCode::OK,
    Json(serde_json::json!({
        "message": "Hello, World!"
    })),
  )
}
