use domain::measurement::uom::Model;

use super::list_paginated_uoms::ListPaginatedUomsError;

pub trait MeasurementServiceTrait {
  fn list_paginated_uoms(
    &self,
    page: u64,
    page_size: u64,
  ) -> Result<Vec<Model>, ListPaginatedUomsError>;
}

pub struct MeasurementService {
  pub measurement_service: dyn MeasurementServiceTrait + Send + Sync,
}
