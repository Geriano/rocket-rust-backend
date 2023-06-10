use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, FromFormField)]
pub enum PaginationSortBy {
  Asc, Desc
}

#[derive(Clone, Debug, Deserialize, Serialize, FromForm)]
pub struct PaginationRequest {
  pub page: Option<i64>,
  pub limit: Option<i64>,
  pub order: Option<String>,
  pub sort: Option<PaginationSortBy>,
  pub search: Option<String>,
}