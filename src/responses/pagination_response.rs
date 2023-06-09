use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginationResponse<T> {
  page: i64,
  total: i64,
  count: i64,
  from: i64,
  to: i64,
  data: Vec<T>
}
