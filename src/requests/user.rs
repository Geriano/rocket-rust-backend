use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema, openapi::schema};

#[derive(Clone, Debug, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct UserStoreRequest {
  #[schema(example = "Name")]
  pub name: String,

  #[schema(example = "email@local.app")]
  pub email: String,

  #[schema(example = "Username")]
  pub username: String,

  #[schema(example = "LetMeIn")]
  pub password: String,

  #[schema(example = "LetMeIn")]
  pub password_confirmation: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct UserUpdateGeneralInformationRequest {
  #[schema(example = "Name")]
  pub name: String,

  #[schema(example = "email@local.app")]
  pub email: String,

  #[schema(example = "Username")]
  pub username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct UserUpdatePasswordRequest {
  #[schema(example = "LetMeIn111")]
  pub old_password: String,

  #[schema(example = "LetMeIn123")]
  pub password: String,

  #[schema(example = "LetMeIn123")]
  pub password_confirmation: String,
}
