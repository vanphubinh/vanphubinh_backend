use axum::{
  async_trait,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::measurement::uom::ActiveModel as Uom;
use infra::{response::OkResponse, usecase::InvokableUsecase, util::error};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use serde::Deserialize;
use thiserror::Error;

pub struct CreateUomUsecase {
  pub db: DatabaseConnection,
}
#[async_trait]
impl InvokableUsecase for CreateUomUsecase {
  type Error = CreateUomError;
  type Output = OkResponse;
  type Params = CreateUomParams;

  async fn invoke(&self, params: Self::Params) -> Result<Self::Output, Self::Error> {
    let uom = Uom {
      name: Set(params.name.to_owned()),
      ..Default::default()
    };
    uom.insert(&self.db).await?;
    Ok(OkResponse { ok: true })
  }
}

#[derive(Debug, Deserialize)]
pub struct CreateUomParams {
  pub name: String,
}

#[derive(Error, Debug)]
pub enum CreateUomError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for CreateUomError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      CreateUomError::InternalServerError(_) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from(self.to_string()),
      ),
    };

    (status, error(code, Some("create_uom".to_string()))).into_response()
  }
}
