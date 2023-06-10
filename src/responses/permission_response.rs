
use rocket::serde::json::Json;

use crate::{models::{Permission, PermissionUser, PermissionRole}, oas::PermissionOAS};

use super::{ResponseMethod, Response};

impl ResponseMethod<PermissionOAS> for Result<Permission, diesel::result::Error> {
  fn error(self) -> super::CustomJsonResponse {
    Response::from_error(self.err())
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> rocket::serde::json::Json<PermissionOAS> {
    Json(self.unwrap().into())
  }
}

impl ResponseMethod<PermissionOAS> for Permission {
  fn error(self) -> super::CustomJsonResponse {
    Response::internal_server_error(
      "INTERNAL SERVER ERROR".to_string()
    )
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<PermissionOAS> {
    Json(self.into())
  }
}

impl ResponseMethod<Vec<Permission>> for Vec<Permission> {
  fn error(self) -> super::CustomJsonResponse {
    Response::internal_server_error(
      "INTERNAL SERVER ERROR".to_string()
    )
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<Vec<Permission>> {
    Json(self)
  }
}

impl ResponseMethod<Vec<Permission>> for Result<Vec<Permission>, diesel::result::Error> {
  fn error(self) -> super::CustomJsonResponse {
    Response::from_error(self.err())
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<Vec<Permission>> {
    Json(self.unwrap())
  }
}

impl ResponseMethod<Vec<PermissionUser>> for Vec<PermissionUser> {
  fn error(self) -> super::CustomJsonResponse {
    Response::internal_server_error(
      "INTERNAL SERVER ERROR".to_string()
    )
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<Vec<PermissionUser>> {
    Json(self)
  }
}

impl ResponseMethod<Vec<PermissionUser>> for Result<Vec<PermissionUser>, diesel::result::Error> {
  fn error(self) -> super::CustomJsonResponse {
    Response::from_error(self.err())
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<Vec<PermissionUser>> {
    Json(self.unwrap())
  }
}

impl ResponseMethod<Vec<PermissionRole>> for Vec<PermissionRole> {
  fn error(self) -> super::CustomJsonResponse {
    Response::internal_server_error(
      "INTERNAL SERVER ERROR".to_string()
    )
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<Vec<PermissionRole>> {
    Json(self)
  }
}

impl ResponseMethod<Vec<PermissionRole>> for Result<Vec<PermissionRole>, diesel::result::Error> {
  fn error(self) -> super::CustomJsonResponse {
    Response::from_error(self.err())
  }

  fn not_found(self) -> super::CustomJsonResponse {
    Response::not_found()
  }

  fn success(self) -> Json<Vec<PermissionRole>> {
    Json(self.unwrap())
  }
}
