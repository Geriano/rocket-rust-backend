use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, openapi::schema};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, FromFormField, ToSchema)]
pub enum PaginationSortBy {
  Asc, Desc
}

#[derive(Clone, Debug, Deserialize, Serialize, FromForm, ToSchema)]
pub struct PaginationRequest {
  #[schema(example = 1)]
  pub page: Option<i64>,

  #[schema(example = 10)]
  pub limit: Option<i64>,

  #[schema(example = "created_at")]
  pub order: Option<String>,

  #[schema(example = "asc", value_type = String, required = false)]
  pub sort: Option<PaginationSortBy>,

  #[schema(example = "TeSt")]
  pub search: Option<String>,
}