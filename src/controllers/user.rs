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
  schemas::users,
  oas::UserOAS,
};

#[get("/?<page>&<limit>&<search>")]
pub fn pagination(
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

#[post("/", data = "<request>")]
pub fn store(request: Json<UserStoreRequest>) -> JsonResponse<UserOAS> {
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

#[get("/<id>")]
pub fn show(id: String) -> JsonResponse<UserOAS> {
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

#[put("/<id>", data = "<request>")]
pub fn update(id: String, request: Json<UserUpdateGeneralInformationRequest>) -> JsonResponse<UserOAS> {
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

#[put("/<id>/password", data = "<request>")]
pub fn update_password(id: String, request: Json<UserUpdatePasswordRequest>) -> JsonResponse<UserOAS> {
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

#[delete("/<id>")]
pub fn delete(id: String) -> JsonResponse<UserOAS> {
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

#[delete("/<id>/purge")]
pub fn purge(id: String) -> AppResponse<()> {
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
