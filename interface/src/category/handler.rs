use std::sync::Arc;

use axum::extract::{Query, State};
use axum_macros::debug_handler;
use domain::product::category::PartialModel as Category;
use infra::usecase::InvokableUsecase;
use infra::{response::PaginatedResponse, state::AppState};
use service::product::list_paginated_categories::{
  ListPaginatedCategoriesError, ListPaginatedCategoriesParams, ListPaginatedCategoriesUsecase,
};

#[debug_handler]
pub async fn list_paginated_categories(
  State(state): State<Arc<AppState>>,
  Query(query): Query<ListPaginatedCategoriesParams>,
) -> Result<PaginatedResponse<Category>, ListPaginatedCategoriesError> {
  let usecase = ListPaginatedCategoriesUsecase {
    db: state.db.clone(),
  };

  let (categories, meta) = usecase.invoke(query).await?;

  Ok(PaginatedResponse::<Category> {
    ok: true,
    data: categories,
    meta,
  })
}
