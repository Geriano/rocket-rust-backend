use chrono::Utc;
use diesel::{QueryDsl, PgTextExpressionMethods, RunQueryDsl, ExpressionMethods};
use rocket::{serde::json::Json, Route};
use uuid::Uuid;

use crate::{
  helpers::{
    get_conn, 
    Hash
  }, 
  models::User, 
  requests::{
    UserStoreRequest, 
    UserUpdateGeneralInformationRequest, 
    UserUpdatePasswordRequest
  }, 
  responses::{
    Pagination, 
    Response, 
    ResponseMethod, 
    PaginationResponse,
    JsonResponse, AppResponse,
  },
  oas::{
    UserOAS,
    UserPaginationOAS,
  },
  schemas::users,
  middleware::Authentication,
};

/// Get pagination of user
#[utoipa::path(
  get,
  tag = "Master User",
  path = "/api/v1/user/",
  responses(
    (status = 200, description = "OK", body = UserPaginationOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  )
)]
#[get("/?<page>&<limit>&<search>")]
pub async fn pagination(
  authentication: Authentication, 
  page: Option<i64>,
  limit: Option<i64>,
  search: Option<String>
) -> PaginationResponse<UserOAS> {
  let page = page.unwrap_or(1);
  let limit = limit.unwrap_or(10);
  let offset = (page - 1) * limit;

  let result: Result<Vec<User>, diesel::result::Error>;
  let count: Result<i64, diesel::result::Error>;
  let mut conn = get_conn();
  let query = users::table.filter(users::deleted_at.is_null());

  if search.is_some() {
    let search = format!("%{}%", search.unwrap().to_lowercase());
    let query = query.filter(users::name.ilike(search.clone()))
      .filter(users::email.ilike(search.clone()))
      .filter(users::username.ilike(search.clone()));

    count = query.clone()
      .count()
      .get_result(&mut conn);

    result = query.limit(limit)
      .offset(offset)
      .get_results(&mut conn);
  } else {
    count = query.clone()
      .count()
      .get_result(&mut conn);

    result = query.limit(limit)
      .offset(offset)
      .get_results(&mut conn);
  }

  if count.is_err() {
    return Err(Response::from_error(
      count.err()
    ));
  }

  if result.is_err() {
    return Err(Response::from_error(
      result.err()
    ));
  }

  let count = count.unwrap();

  Ok(Json(
    Pagination {
      page: page,
      total: count / limit,
      from: (page - 1) * limit,
      to: page * limit,
      count: count,
      data: result.unwrap()
        .iter()
        .map(|user| {
          let oas: UserOAS = user.clone().into();

          oas
        })
        .collect(),
    }
  ))
}

/// Create user
#[utoipa::path(
  post,
  path = "/api/v1/user/",
  tag = "Master User",
  responses(
    (status = 200, description = "OK", body = UserOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  request_body = UserStoreRequest,
)]
#[post("/", data = "<request>")]
pub async fn store(authentication: Authentication, request: Json<UserStoreRequest>) -> JsonResponse<UserOAS> {
  if request.name.is_empty() || request.email.is_empty() || request.username.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();

  if User::find_by_email(&mut conn, request.email.clone()).is_ok() {
    return Err(Response::bad_request(
      "DATA EXIST".to_string()
    ));
  }

  if User::find_by_username(&mut conn, request.username.clone()).is_ok() {
    return Err(Response::bad_request(
      "DATA EXIST".to_string()
    ));
  }

  if !request.password.eq(&request.password_confirmation) {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let id = Uuid::new_v4().to_string();
  let mut hash = Hash::new(id.clone());
  let user = User {
    id,
    name: request.name.to_lowercase(),
    email: request.email.to_lowercase(),
    email_verified_at: None,
    username: request.username.to_lowercase(),
    password: hash.append(request.password.clone()).digest(),
    created_at: Utc::now().naive_local(),
    updated_at: Utc::now().naive_local(),
    deleted_at: None,
  };

  let user = user.insert(&mut conn);

  if user.is_err() {
    return Err(Response::from_error(
      user.err()
    ));
  }

  Ok(user.success())
}

/// Show user by id
#[utoipa::path(
  get,
  path = "/api/v1/user/{id}",
  tag = "Master User",
  responses(
    (status = 200, description = "OK", body = UserOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  params(
    ("id" = String, Path, description = "User ID"),
  )
)]
#[get("/<id>")]
pub async fn show(authentication: Authentication, id: String) -> JsonResponse<UserOAS> {
  if id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let user = User::find(&mut conn, id);

  if user.is_err() {
    return Err(user.not_found());
  }

  Ok(user.success())
}

/// Update user by id
#[utoipa::path(
  put,
  path = "/api/v1/user/{id}",
  tag = "Master User",
  responses(
    (status = 200, description = "OK", body = UserOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  params(
    ("id" = String, Path, description = "User ID"),
  ),
)]
#[put("/<id>", data = "<request>")]
pub async fn update(authentication: Authentication, id: String, request: Json<UserUpdateGeneralInformationRequest>) -> JsonResponse<UserOAS> {
  if id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let user = User::find(&mut conn, id);

  if user.is_err() {
    return Err(user.not_found());
  }

  let mut user = user.unwrap();

  if !user.email.eq(&request.email) {
    let exist = User::find_by_email(&mut conn, request.email.clone());

    if exist.is_ok() && !exist.unwrap().id.eq(&user.id) {
      return Err(Response::bad_request(
        "DATA EXIST".to_string()
      ));
    }
  }

  if !user.username.eq(&request.username) {
    let exist = User::find_by_username(&mut conn, request.username.clone());

    if exist.is_ok() && !exist.unwrap().id.eq(&user.id) {
      return Err(Response::bad_request(
        "DATA EXIST".to_string()
      ));
    }
  }

  user.name = request.name.to_lowercase();
  user.email = request.email.to_lowercase();
  user.username = request.username.to_lowercase();

  let user = user.update(&mut conn);

  if user.is_err() {
    return Err(user.error());
  }

  Ok(user.success())
}

/// Update password user by id
#[utoipa::path(
  put,
  path = "/api/v1/user/{id}/password",
  tag = "Master User",
  responses(
    (status = 200, description = "OK", body = UserOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  params(
    ("id" = String, Path, description = "User ID"),
  )
)]
#[put("/<id>/password", data = "<request>")]
pub async fn update_password(authentication: Authentication, id: String, request: Json<UserUpdatePasswordRequest>) -> JsonResponse<UserOAS> {
  if id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  if !request.password.eq(&request.password_confirmation) {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let user = User::find(&mut conn, id);

  if user.is_err() {
    return  Err(user.not_found());
  }

  let mut user = user.unwrap();
  let hash = Hash::new(user.id.clone()).append(request.old_password.clone()).digest();

  if !user.password.eq(&hash) {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  user.password = Hash::new(user.id.clone()).append(request.password.clone()).digest();

  let user = user.update(&mut conn);

  if user.is_err() {
    return Err(user.error());
  }

  Ok(user.success())
}

/// Soft delete user by id
#[utoipa::path(
  delete,
  path = "/api/v1/user/{id}",
  tag = "Master User",
  responses(
    (status = 200, description = "OK", body = UserOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  params(
    ("id" = String, Path, description = "User ID"),
  )
)]
#[delete("/<id>")]
pub async fn delete(authentication: Authentication, id: String) -> JsonResponse<UserOAS> {
  if id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let user = User::find(&mut conn, id);

  if user.is_err() {
    return Err(user.not_found());
  }

  let user = user.unwrap().delete(&mut conn);

  if user.is_err() {
    return Err(user.error());
  }

  Ok(user.success())
}

/// Hard delete user by id
#[utoipa::path(
  delete,
  path = "/api/v1/user/{id}/purge",
  tag = "Master User",
  responses(
    (status = 200, description = "OK", body = UserOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  params(
    ("id" = String, Path, description = "User ID"),
  ),
)]
#[delete("/<id>/purge")]
pub async fn purge(authentication: Authentication, id: String) -> AppResponse<()> {
  if id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let result = User::find(&mut conn, id);

  if result.is_err() {
    return Err(result.not_found());
  }

  let deleted = result.unwrap().purge(&mut conn);

  if deleted.is_err() {
    return Err(Response::from_error(
      deleted.err()
    ));
  }

  Ok(())
}

pub fn routes() -> Vec<Route> {
  routes![pagination, store, show, update, update_password, delete, purge]
}
