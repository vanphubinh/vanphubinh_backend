use std::sync::Arc;

use axum::{routing::get, Router};
use infra::state::AppState;

use super::handler::list_paginated_uoms;
pub struct UomRouter {}

impl UomRouter {
  pub fn new() -> Router<Arc<AppState>> {
    Router::new().route("/uoms.list", get(list_paginated_uoms))
  }
}
