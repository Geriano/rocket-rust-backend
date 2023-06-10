mod pagination;
mod user;
mod permission;
mod role;
mod auth;

use rocket::{response::status, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

pub use pagination::Pagination;
pub use auth::AuthenticatedResponse;

pub type CustomJsonResponse = status::Custom<Json<Response>>;
pub type AppResponse<T> = Result<T, CustomJsonResponse>;
pub type JsonResponse<T> = AppResponse<Json<T>>;
pub type PaginationResponse<T> = JsonResponse<Pagination<T>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
  pub message: String,
}

impl Response {
  pub fn new(status: Status, message: String) -> CustomJsonResponse {
    status::Custom(status, Json(
      Response {
        message
      }
    ))
  }

  pub fn unauthorize() -> CustomJsonResponse {
    Response::new(Status::Unauthorized, "Unauthorized".to_string())
  }

  pub fn bad_request(message: String) -> CustomJsonResponse {
    Response::new(Status::BadRequest, message)
  }

  pub fn internal_server_error(message: String) -> CustomJsonResponse {
    Response::new(Status::InternalServerError, message)
  }

  pub fn not_found() -> CustomJsonResponse {
    Response::new(Status::NotFound, "NOT FOUND".to_string())
  }

  pub fn from_error(e: Option<diesel::result::Error>) -> CustomJsonResponse {
    Response::internal_server_error(e.unwrap().to_string())
  }
}

pub trait ResponseMethod<T> {
  fn success(self) -> Json<T>;
  fn error(self) -> CustomJsonResponse;
  fn not_found(self) -> CustomJsonResponse;
}
