use axum::async_trait;
use sea_orm::ConnectionTrait;

#[async_trait]
pub trait InvokableUsecase<T>
where
  T: ConnectionTrait,
{
  type Error;
  type Output;
  type Params;

  async fn invoke(&self, params: Self::Params) -> Result<Self::Output, Self::Error>;
}
