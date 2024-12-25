use axum::{
  async_trait,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::category::{self, Entity as Category};
use infra::{response::PaginationMeta, usecase::InvokableUsecase, util::error};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};
use serde::Deserialize;
use thiserror::Error;

pub struct ListPaginatedCategoriesUsecase {
  pub db: DatabaseConnection,
}

#[async_trait]
impl InvokableUsecase for ListPaginatedCategoriesUsecase {
  type Error = ListPaginatedCategoriesError;
  type Output = (Vec<category::PartialModel>, PaginationMeta);
  type Params = ListPaginatedCategoriesParams;

  async fn invoke(&self, params: Self::Params) -> Result<Self::Output, Self::Error> {
    let per_page = params.per_page.unwrap_or(30);
    let page = params.page.unwrap_or(1) - 1;

    let category_pages = Category::find()
      .into_partial_model::<category::PartialModel>()
      .paginate(&self.db, per_page);
    let categories = category_pages.fetch_page(page).await?;
    let items_and_pages = category_pages.num_items_and_pages().await?;
    let total = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    Ok((
      categories,
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
pub struct ListPaginatedCategoriesParams {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

#[derive(Error, Debug)]
pub enum ListPaginatedCategoriesError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListPaginatedCategoriesError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      ListPaginatedCategoriesError::InternalServerError(_) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from(self.to_string()),
      ),
    };

    (
      status,
      error(code, Some("list_paginated_categories".to_string())),
    )
      .into_response()
  }
}
