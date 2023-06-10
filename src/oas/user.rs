use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::User;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserOAS {
  pub id: String,
  pub name: String,
  pub email: String,
  pub email_verified_at: Option<NaiveDateTime>,
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
