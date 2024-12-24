use std::{net::SocketAddr, sync::Arc};

use axum::{
  http::{header::CONTENT_TYPE, Method, StatusCode},
  routing::get,
  Router,
};
use infra::state::AppState;
use interface::uom::route::UomRouter;
use sea_orm::{Database, DatabaseConnection, DbErr};
use tokio::net::TcpListener;
use tower_http::{
  cors::{Any, CorsLayer},
  trace::{self, TraceLayer},
};
use tracing::Level;

#[tokio::main]
pub async fn start() {
  dotenvy::dotenv().ok();

  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_test_writer()
    .init();

  tracing::info!("Connecting to database...");

  let db = match connect_db("DATABASE_URL").await {
    Ok(db) => {
      tracing::info!("Connected to database!");
      db
    }
    Err(_) => {
      tracing::error!("Failed to connect to database!");
      return;
    }
  };

  let cors = CorsLayer::new()
    .allow_methods([
      Method::GET,
      Method::POST,
      Method::OPTIONS,
      Method::PUT,
      Method::DELETE,
    ])
    .allow_origin(Any)
    .allow_headers([CONTENT_TYPE]);

  let app_state = Arc::new(AppState::new(db));

  let app = Router::new()
    .route("/", get(|| async { "This is API of Van Phu Binh" }))
    .merge(UomRouter::new())
    .layer(cors)
    .layer(
      TraceLayer::new_for_http().make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO)),
    )
    .with_state(app_state.clone());

  let app = app.fallback((StatusCode::NOT_FOUND, "Not found"));

  let port: u16 = std::env::var("PORT")
    .unwrap_or("3000".into())
    .parse()
    .expect("Failed to convert to number");

  let ipv6 = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
  let ipv6_listener = TcpListener::bind(&ipv6).await.unwrap();

  tracing::info!("Listening on IPv6 at {}!", ipv6);

  axum::serve(ipv6_listener, app).await.unwrap();
}

async fn connect_db(env_var: &str) -> Result<DatabaseConnection, DbErr> {
  let db_url = std::env::var(env_var).unwrap();
  let db = Database::connect(&db_url).await?;
  Ok(db)
}
