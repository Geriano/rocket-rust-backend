use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, openapi::schema};

use crate::models::User;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct UserOAS {
  #[schema(example = "f6943706-7a06-4b62-840d-05327e6c6b3b")]
  pub id: String,

  #[schema(example = "Name")]
  pub name: String,

  #[schema(example = "email@local.app")]
  pub email: String,

  #[schema(value_type = String, example = "2023-06-11", format = Date, nullable = true)]
  pub email_verified_at: Option<NaiveDateTime>,

  #[schema(example = "Username")]
  pub username: String,
}

impl From<User> for UserOAS {
  fn from(user: User) -> Self {
    Self {
      id: user.id,
      name: user.name,
      email: user.email,
      email_verified_at: user.email_verified_at,
      username: user.username,
    }
  }
}
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct UserPaginationOAS {
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

  #[schema(value_type = [UserOAS])]
  pub data: Vec<UserOAS>
}

