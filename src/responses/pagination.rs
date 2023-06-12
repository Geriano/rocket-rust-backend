use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, openapi::schema};
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Pagination<T> {
  #[schema(example = 1)]
  pub page: i64,

  #[schema(example = 1)]
  pub total: i64,

  #[schema(example = 1)]
  pub count: i64,

  #[schema(example = 1)]
  pub from: i64,

  #[schema(example = 1)]
  pub to: i64,

  #[schema(value_type = [T])]
  pub data: Vec<T>
}
