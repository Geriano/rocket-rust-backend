use chrono::Utc;
use rocket::{
  Route,
  serde::json::Json
};
use uuid::Uuid;

use crate::{
  responses::{
    JsonResponse,
    Response,
    ResponseMethod
  },
  models::{
    Permission,
    User,
    PermissionUser,
    PermissionRole,
    Role
  },
  requests::{
    PermissionCreateRequest,
    SyncPermissionToUser,
    SyncPermissionToRole,
    PermissionUpdateRequest
  },
  helpers::get_conn,
  oas::PermissionOAS, middleware::Authentication
};

#[get("/")]
pub async fn all(authentication: Authentication) -> JsonResponse<Vec<PermissionOAS>> {
  let mut conn = get_conn();
  let permissions = Permission::all(&mut conn);

  if permissions.is_err() {
    return Err(permissions.error());
  }

  Ok(Json(
    permissions.unwrap()
      .iter()
      .map(|permission| permission.clone().into())
      .collect()
  ))
}

#[post("/", data = "<request>")]
pub async fn create(authentication: Authentication, request: Json<PermissionCreateRequest>) -> JsonResponse<PermissionOAS> {
  if request.code.is_empty() || request.name.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let exists = Permission::find_by_code(&mut conn, request.code.clone());

  if exists.is_ok() {
    return Err(Response::bad_request(
      "DATA EXIST".to_string()
    ));
  }

  let permission = Permission {
    id: Uuid::new_v4().to_string(),
    code: request.code.clone(),
    name: request.name.clone(),
    created_at: Utc::now().naive_local(),
    updated_at: Utc::now().naive_local(),
  };

  let permission = permission.insert(&mut conn);

  if permission.is_err() {
    return Err(Response::from_error(
      permission.err()
    ));
  }

  Ok(permission.success())
}

#[get("/<id>")]
pub async fn show(authentication: Authentication, id: String) -> JsonResponse<PermissionOAS> {
  if id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let permission = Permission::find(&mut conn, id.clone());

  if permission.is_err() {
    return Err(permission.not_found());
  }

  Ok(permission.success())
}

#[put("/<id>", data = "<request>")]
pub async fn update(authentication: Authentication, id: String, request: Json<PermissionUpdateRequest>) -> JsonResponse<PermissionOAS> {
  if id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let permission = Permission::find(&mut conn, id.clone());

  if permission.is_err() {
    return Err(permission.not_found());
  }

  let mut permission = permission.unwrap();

  permission.name = request.name.clone();

  let permission = permission.update(&mut conn);

  if permission.is_err() {
    return Err(permission.error());
  }

  Ok(permission.success())
}

#[delete("/<id>")]
pub async fn delete(authentication: Authentication, id: String) -> JsonResponse<PermissionOAS> {
  if id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let permission = Permission::find(&mut conn, id.clone());

  if permission.is_err() {
    return Err(permission.not_found());
  }

  let permission = permission.unwrap();
  let deleted = permission.delete(&mut conn);

  if deleted.is_err() {
    return Err(Response::from_error(
      deleted.err()
    ));
  }

  Ok(permission.success())
}

#[put("/sync-user", data = "<request>")]
pub async fn sync_permission_user(authentication: Authentication, request: Json<SyncPermissionToUser>) -> JsonResponse<Vec<Permission>> {
  if request.user_id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let user = User::find(&mut conn, request.user_id.clone());

  if user.is_err() {
    return Err(user.error());
  }

  let user = user.unwrap();
  let deleted = PermissionUser::delete_by_user(&mut conn, user.id.clone());

  if deleted.is_err() {
    return Err(Response::from_error(
      deleted.err()
    ));
  }

  let permission_user: Vec<PermissionUser> = request.permissions
    .clone()
    .iter()
    .map(|permission| PermissionUser {
      id: Uuid::new_v4().to_string(),
      permission_id: permission.clone(),
      user_id: user.id.clone(),
      created_at: Utc::now().naive_local(),
    })
    .collect();
  
  let permission_user = PermissionUser::insert_many(
    &mut conn, permission_user.clone()
  );

  if permission_user.is_err() {
    return Err(permission_user.error());
  }

  let permission_user = permission_user.unwrap();
  let permissions = Permission::id_in(
    &mut conn, permission_user.iter().map(|permission_user| permission_user.permission_id.clone()).collect()
  );

  if permissions.is_err() {
    return Err(permissions.error());
  }

  Ok(permissions.success())
}

#[put("/sync-role", data = "<request>")]
pub async fn sync_permission_role(authentication: Authentication, request: Json<SyncPermissionToRole>) -> JsonResponse<Vec<Permission>> {
  if request.role_id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let role = Role::find(&mut conn, request.role_id.clone());

  if role.is_err() {
    return Err(Response::bad_request(
      "DATA NOT EXIST".to_string()
    ));
  }

  let role = role.unwrap();
  let deleted = PermissionRole::delete_by_role(&mut conn, role.id.clone());

  if deleted.is_err() {
    return Err(Response::from_error(
      deleted.err()
    ));
  }

  let permission_role: Vec<PermissionRole> = request.permissions
    .clone()
    .iter()
    .map(|permission| PermissionRole {
      id: Uuid::new_v4().to_string(),
      permission_id: permission.clone(),
      role_id: role.id.clone(),
      created_at: Utc::now().naive_local(),
    })
    .collect();

  let permission_role = PermissionRole::insert_many(
    &mut conn, permission_role.clone()
  );

  if permission_role.is_err() {
    return Err(permission_role.error());
  }

  let permission_role = permission_role.unwrap();
  let permissions = Permission::id_in(
    &mut conn, permission_role.iter().map(|permission_role| permission_role.permission_id.clone()).collect()
  );

  if permissions.is_err() {
    return Err(permissions.error());
  }

  Ok(permissions.success())
}

pub fn routes() -> Vec<Route> {
  routes![all, create, show, update, delete, sync_permission_user, sync_permission_role]
}
