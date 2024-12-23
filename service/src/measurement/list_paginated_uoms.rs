use axum::{
  async_trait,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::measurement::uom::{self, Entity as Uom};
use infra::{response::PaginationMeta, usecase::InvokableUsecase, util::error};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};
use serde::Deserialize;
use thiserror::Error;

pub struct ListPaginatedUomsUsecase {
  pub db: DatabaseConnection,
}

#[async_trait]
impl InvokableUsecase<DatabaseConnection> for ListPaginatedUomsUsecase {
  type Error = ListPaginatedUomsError;
  type Output = (Vec<uom::PartialModel>, PaginationMeta);
  type Params = ListPaginatedUomsParams;

  async fn invoke(&self, params: Self::Params) -> Result<Self::Output, Self::Error> {
    let per_page = params.per_page.unwrap_or(30);
    let page = params.page.unwrap_or(1) - 1;

    let uom_pages = Uom::find()
      .into_partial_model::<uom::PartialModel>()
      .paginate(&self.db, per_page);
    let uoms = uom_pages.fetch_page(page).await?;
    let items_and_pages = uom_pages.num_items_and_pages().await?;
    let total = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    Ok((
      uoms,
      PaginationMeta {
        total,
        total_pages,
        page: page + 1,
        per_page,
      },
    ))
  }
}

#[derive(Debug, Deserialize)]
pub struct ListPaginatedUomsParams {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

#[derive(Error, Debug)]
pub enum ListPaginatedUomsError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListPaginatedUomsError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      ListPaginatedUomsError::InternalServerError(_) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from(self.to_string()),
      ),
    };

    (status, error(code, Some("list_paginated_uoms".to_string()))).into_response()
  }
}
