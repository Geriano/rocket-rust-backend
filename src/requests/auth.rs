use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginRequest {
  pub email_or_username: String,
  pub password: String,
}
