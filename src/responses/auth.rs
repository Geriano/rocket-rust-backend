use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::oas::UserOAS;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthenticatedResponse {
  pub user: UserOAS,
  pub token: String,
  pub expired_at: Option<NaiveDateTime>,
}
