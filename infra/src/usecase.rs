use axum::async_trait;

#[async_trait]
pub trait InvokableUsecase {
  type Error;
  type Output;
  type Params;

  async fn invoke(&self, params: Self::Params) -> Result<Self::Output, Self::Error>;
}
