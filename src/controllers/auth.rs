use diesel::result::Error;
use rocket::{serde::json::Json, Route};

use crate::{
  requests::LoginRequest,
  responses::{
    JsonResponse,
    AuthenticatedResponse,
    Response,
    AppResponse
  },
  models::{
    User,
    ApiToken
  },
  helpers::get_conn,
  middleware::Authentication,
  oas::UserOAS
};

/// Authenticate email or user with password
#[utoipa::path(
  post,
  tag = "Authentication",
  request_body = LoginRequest,
  path = "/api/v1/auth/",
  responses(
    (status = 200, description = "OK", body = AuthenticatedResponse),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
)]
#[post("/", data = "<request>")]
pub async fn login(request: Json<LoginRequest>) -> JsonResponse<AuthenticatedResponse> {
  if request.email_or_username.is_empty() || request.password.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let mut user: Result<User, Error> = User::find_by_email(&mut conn, request.email_or_username.clone());

  if user.is_err() {
    user = User::find_by_username(&mut conn, request.email_or_username.clone());

    if user.is_err() {
      return Err(Response::bad_request(
        "DATA NOT EXIST".to_string()
      ));
    }
  }

  let user = user.unwrap();
  let token = ApiToken::create(&mut conn, user.clone().id, None);

  Ok(Json(
    AuthenticatedResponse {
      user: user.into(),
      token,
      expired_at: None,
    }
  ))
}

/// Get current authenticated user
#[utoipa::path(
  get,
  tag = "Authentication",
  path = "/api/v1/auth/",
  responses(
    (status = 200, description = "OK", body = UserOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  security(("token" = [])),
)]
#[get("/")]
pub async fn user(authentication: Authentication) -> JsonResponse<UserOAS> {
  let user: UserOAS = authentication.user.into();

  Ok(Json(user))
}

/// Revoke token current authenticated user
#[utoipa::path(
  delete,
  tag = "Authentication",
  path = "/api/v1/auth/",
  responses(
    (status = 200, description = "OK"),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  security(("token" = [])),
)]
#[delete("/")]
pub async fn logout(authentication: Authentication) -> AppResponse<()> {
  let mut conn = get_conn();
  let token = ApiToken::find(&mut conn, authentication.token);

  if token.is_err() {
    return Err(Response::unauthorize())
  }
  
  let deleted = ApiToken::delete(&mut conn, token.unwrap().id);

  if deleted.is_err() {
    return Err(Response::from_error(
      deleted.err()
    ));
  }
  
  Ok(())
}

pub fn routes() -> Vec<Route> {
  routes![login, user, logout]
}
