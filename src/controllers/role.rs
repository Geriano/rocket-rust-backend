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
    Role,
    User,
    RoleUser,
    PermissionRole
  },
  requests::{
    PermissionCreateRequest,
    SyncRoleToUser,
    SyncPermissionToRole,
    PermissionUpdateRequest
  },
  helpers::get_conn,
  oas::RoleOAS, middleware::Authentication
};

#[get("/")]
pub async fn all(authentication: Authentication) -> JsonResponse<Vec<RoleOAS>> {
  let mut conn = get_conn();
  let roles = Role::all(&mut conn);

  if roles.is_err() {
    return Err(roles.error());
  }

  Ok(Json(
    roles.unwrap()
      .iter()
      .map(|role| role.clone().into())
      .collect()
  ))
}

#[post("/", data = "<request>")]
pub async fn create(authentication: Authentication, request: Json<PermissionCreateRequest>) -> JsonResponse<RoleOAS> {
  if request.code.is_empty() || request.name.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let exists = Role::find_by_code(&mut conn, request.code.clone());

  if exists.is_ok() {
    return Err(Response::bad_request(
      "DATA EXIST".to_string()
    ));
  }

  let role = Role {
    id: Uuid::new_v4().to_string(),
    code: request.code.clone(),
    name: request.name.clone(),
    created_at: Utc::now().naive_local(),
    updated_at: Utc::now().naive_local(),
  };

  let role = role.insert(&mut conn);

  if role.is_err() {
    return Err(Response::from_error(
      role.err()
    ));
  }

  Ok(role.success())
}

#[get("/<id>")]
pub async fn show(authentication: Authentication, id: String) -> JsonResponse<RoleOAS> {
  if id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let role = Role::find(&mut conn, id.clone());

  if role.is_err() {
    return Err(role.not_found());
  }

  Ok(role.success())
}

#[put("/<id>", data = "<request>")]
pub async fn update(authentication: Authentication, id: String, request: Json<PermissionUpdateRequest>) -> JsonResponse<RoleOAS> {
  if id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let role = Role::find(&mut conn, id.clone());

  if role.is_err() {
    return Err(role.not_found());
  }

  let mut role = role.unwrap();

  role.name = request.name.clone();

  let role = role.update(&mut conn);

  if role.is_err() {
    return Err(role.error());
  }

  Ok(role.success())
}

#[delete("/<id>")]
pub async fn delete(authentication: Authentication, id: String) -> JsonResponse<RoleOAS> {
  if id.is_empty() {
    return Err(Response::bad_request(
      "BAD REQUEST".to_string()
    ));
  }

  let mut conn = get_conn();
  let role = Role::find(&mut conn, id.clone());

  if role.is_err() {
    return Err(role.not_found());
  }

  let role = role.unwrap();
  let deleted = role.delete(&mut conn);

  if deleted.is_err() {
    return Err(Response::from_error(
      deleted.err()
    ));
  }

  Ok(role.success())
}

#[put("/sync-user", data = "<request>")]
pub async fn sync_permission_user(authentication: Authentication, request: Json<SyncRoleToUser>) -> JsonResponse<Vec<Role>> {
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
  let deleted = RoleUser::delete_by_user(&mut conn, user.id.clone());

  if deleted.is_err() {
    return Err(Response::from_error(
      deleted.err()
    ));
  }

  let role_user: Vec<RoleUser> = request.roles
    .clone()
    .iter()
    .map(|permission| RoleUser {
      id: Uuid::new_v4().to_string(),
      role_id: permission.clone(),
      user_id: user.id.clone(),
      created_at: Utc::now().naive_local(),
    })
    .collect();
  
  let role_user = RoleUser::insert_many(
    &mut conn, role_user.clone()
  );

  if role_user.is_err() {
    return Err(role_user.error());
  }

  let role_user = role_user.unwrap();
  let permissions = Role::id_in(
    &mut conn, role_user.iter().map(|role_user| role_user.role_id.clone()).collect()
  );

  if permissions.is_err() {
    return Err(permissions.error());
  }

  Ok(permissions.success())
}

#[put("/sync-role", data = "<request>")]
pub async fn sync_permission_role(authentication: Authentication, request: Json<SyncPermissionToRole>) -> JsonResponse<Vec<Role>> {
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
  let permissions = Role::id_in(
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
