use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, openapi::schema};

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginRequest {
  #[schema(example = "test@local.app")]
  pub email_or_username: String,

  #[schema(example = "password")]
  pub password: String,
}
