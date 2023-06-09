use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginationResponse<T> {
  pub page: i64,
  pub total: i64,
  pub count: i64,
  pub from: i64,
  pub to: i64,
  pub data: Vec<T>
}
