use std::net::SocketAddr;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
  let app = Router::new().route("/", get(root));

  let port: u16 = std::env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .expect("Failed to parse PORT");

  let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
  let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

  axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
  "Hello World, from Axum!"
}
