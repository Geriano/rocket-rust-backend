use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserStoreRequest {
  pub name: String,
  pub email: String,
  pub username: String,
  pub password: String,
  pub password_confirmation: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserUpdateGeneralInformationRequest {
  pub name: String,
  pub email: String,
  pub username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserUpdatePasswordRequest {
  pub old_password: String,
  pub password: String,
  pub password_confirmation: String,
}
