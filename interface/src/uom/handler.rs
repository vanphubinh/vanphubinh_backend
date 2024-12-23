use std::sync::Arc;

use axum::extract::{Query, State};
use axum_macros::debug_handler;
use domain::measurement::uom::PartialModel as Uom;
use infra::{response::PaginatedResponse, state::AppState};
use service::measurement::list_paginated_uoms::{
  ListPaginatedUomsError, ListPaginatedUomsParams, ListPaginatedUomsUsecase,
};

#[debug_handler]
pub async fn list_paginated_uoms(
  State(state): State<Arc<AppState>>,
  Query(query): Query<ListPaginatedUomsParams>,
) -> Result<PaginatedResponse<Uom>, ListPaginatedUomsError> {
  let usecase = ListPaginatedUomsUsecase {
    page: Some(query.page.unwrap_or(1)),
    per_page: Some(query.per_page.unwrap_or(30)),
  };

  let (uoms, meta) = usecase.invoke(state.db.clone()).await?;

  Ok(PaginatedResponse::<Uom> {
    ok: true,
    data: uoms,
    meta,
  })
}
