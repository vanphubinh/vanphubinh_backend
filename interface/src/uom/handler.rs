use std::sync::Arc;

use axum::extract::{Json, Query, State};
use axum_macros::debug_handler;
use domain::measurement::uom::PartialModel as Uom;
use infra::response::OkResponse;
use infra::usecase::InvokableUsecase;
use infra::{response::PaginatedResponse, state::AppState};
use service::measurement::create_uom::{CreateUomError, CreateUomParams, CreateUomUsecase};
use service::measurement::list_paginated_uoms::{
  ListPaginatedUomsError, ListPaginatedUomsParams, ListPaginatedUomsUsecase,
};

#[debug_handler]
pub async fn list_paginated_uoms(
  State(state): State<Arc<AppState>>,
  Query(query): Query<ListPaginatedUomsParams>,
) -> Result<PaginatedResponse<Uom>, ListPaginatedUomsError> {
  let usecase = ListPaginatedUomsUsecase {
    db: state.db.clone(),
  };

  let (uoms, meta) = usecase.invoke(query).await?;

  Ok(PaginatedResponse::<Uom> {
    ok: true,
    data: uoms,
    meta,
  })
}

#[debug_handler]
pub async fn create_uom(
  State(state): State<Arc<AppState>>,
  Json(body): Json<CreateUomParams>,
) -> Result<OkResponse, CreateUomError> {
  let usecase = CreateUomUsecase {
    db: state.db.clone(),
  };
  usecase.invoke(body).await
}
