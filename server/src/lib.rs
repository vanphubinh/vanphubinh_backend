use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn start() {
  tracing_subscriber::fmt::init();

  let app = Router::new()
    .route("/", get(root))
    .route("/complex", get(complex));

  let port: u16 = std::env::var("PORT")
    .unwrap_or("3000".into())
    .parse()
    .expect("failed to convert to number");

  let ipv6 = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
  let ipv6_listener = TcpListener::bind(&ipv6).await.unwrap();

  tracing::info!("Listening on IPv6 at {}!", ipv6);

  axum::serve(ipv6_listener, app).await.unwrap();
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
