use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use infra::state::AppState;
use interface::uom::route::UomRouter;
use sea_orm::{Database, DatabaseConnection, DbErr};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn start() {
  dotenvy::dotenv().ok();

  tracing_subscriber::fmt::init();

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

  let app_state = Arc::new(AppState::new(db));

  let app = Router::new()
    .merge(UomRouter::new())
    .with_state(app_state.clone());

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
