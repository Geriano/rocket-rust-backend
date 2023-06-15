use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, openapi::schema};

use crate::oas::UserOAS;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct AuthenticatedResponse {
  #[schema()]
  pub user: UserOAS,

  #[schema(example = "f6943706-7a06-4b62-840d-05327e6c6b3b")]
  pub token: String,

  #[schema(example = "2023-06-15", value_type = String, format = DateTime)]
  pub expired_at: Option<NaiveDateTime>,
}
