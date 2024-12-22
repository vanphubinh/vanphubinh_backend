use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn start() {
  tracing_subscriber::fmt::init();

  let app = Router::new().merge(interface::new());

  let port: u16 = std::env::var("PORT")
    .unwrap_or("3000".into())
    .parse()
    .expect("failed to convert to number");

  let ipv6 = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
  let ipv6_listener = TcpListener::bind(&ipv6).await.unwrap();

  tracing::info!("Listening on IPv6 at {}!", ipv6);

  axum::serve(ipv6_listener, app).await.unwrap();
}
