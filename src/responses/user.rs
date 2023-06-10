use rocket::serde::json::Json;

use crate::{models::User, oas::UserOAS};

use super::{ResponseMethod, Response, CustomJsonResponse};

impl ResponseMethod<UserOAS> for Result<User, diesel::result::Error> {
  fn success(self) -> Json<UserOAS> {
    Json(
      self.and_then(|user| Ok(
        user.into()
      ))
        .unwrap()
    )
  }

  fn error(self) -> CustomJsonResponse {
    Response::from_error(self.err())
  }

  fn not_found(self) -> CustomJsonResponse {
    Response::not_found()
  }
}

