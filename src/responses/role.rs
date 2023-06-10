
use rocket::serde::json::Json;

use crate::{models::{Role, RoleUser}, oas::RoleOAS};

use super::{ResponseMethod, Response};

impl ResponseMethod<RoleOAS> for Result<Role, diesel::result::Error> {
  fn error(self) -> super::CustomJsonResponse {
    Response::from_error(self.err())
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> rocket::serde::json::Json<RoleOAS> {
    Json(self.unwrap().into())
  }
}

impl ResponseMethod<RoleOAS> for Role {
  fn error(self) -> super::CustomJsonResponse {
    Response::internal_server_error(
      "INTERNAL SERVER ERROR".to_string()
    )
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<RoleOAS> {
    Json(self.into())
  }
}

impl ResponseMethod<Vec<Role>> for Vec<Role> {
  fn error(self) -> super::CustomJsonResponse {
    Response::internal_server_error(
      "INTERNAL SERVER ERROR".to_string()
    )
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<Vec<Role>> {
    Json(self)
  }
}

impl ResponseMethod<Vec<Role>> for Result<Vec<Role>, diesel::result::Error> {
  fn error(self) -> super::CustomJsonResponse {
    Response::from_error(self.err())
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<Vec<Role>> {
    Json(self.unwrap())
  }
}

impl ResponseMethod<Vec<RoleUser>> for Vec<RoleUser> {
  fn error(self) -> super::CustomJsonResponse {
    Response::internal_server_error(
      "INTERNAL SERVER ERROR".to_string()
    )
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<Vec<RoleUser>> {
    Json(self)
  }
}

impl ResponseMethod<Vec<RoleUser>> for Result<Vec<RoleUser>, diesel::result::Error> {
  fn error(self) -> super::CustomJsonResponse {
    Response::from_error(self.err())
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<Vec<RoleUser>> {
    Json(self.unwrap())
  }
}
