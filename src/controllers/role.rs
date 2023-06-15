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
    SyncRoleToUser,
    SyncPermissionToRole,
    RoleUpdateRequest,
    RoleCreateRequest
  },
  helpers::get_conn,
  oas::RoleOAS,
  middleware::Authentication
};

/// Get all role
#[utoipa::path(
  get,
  tag = "Master Role",
  path = "/api/v1/role/",
  responses(
    (status = 200, description = "OK", body = [RoleOAS]),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  security(("token" = [])),
)]
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

/// Create role
#[utoipa::path(
  post,
  tag = "Master Role",
  request_body = RoleCreateRequest,
  path = "/api/v1/role/",
  responses(
    (status = 200, description = "OK", body = RoleOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  security(("token" = [])),
)]
#[post("/", data = "<request>")]
pub async fn store(authentication: Authentication, request: Json<RoleCreateRequest>) -> JsonResponse<RoleOAS> {
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

/// Show role by id
#[utoipa::path(
  get,
  tag = "Master Role",
  path = "/api/v1/role/{id}",
  responses(
    (status = 200, description = "OK", body = RoleOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  params(
    ("id" = String, Path, description = "Role Id"),
  ),
  security(("token" = [])),
)]
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

/// Update role by id
#[utoipa::path(
  put,
  tag = "Master Role",
  path = "/api/v1/role/{id}",
  request_body = RoleUpdateRequest,
  responses(
    (status = 200, description = "OK", body = RoleOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  params(
    ("id" = String, Path, description = "Role Id"),
  ),
  security(("token" = [])),
)]
#[put("/<id>", data = "<request>")]
pub async fn update(authentication: Authentication, id: String, request: Json<RoleUpdateRequest>) -> JsonResponse<RoleOAS> {
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

/// Delete role by id
#[utoipa::path(
  delete,
  tag = "Master Role",
  path = "/api/v1/role/{id}",
  responses(
    (status = 200, description = "OK", body = RoleOAS),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  params(
    ("id" = String, Path, description = "Role Id"),
  ),
  security(("token" = [])),
)]
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

/// Sync role with user
#[utoipa::path(
  put,
  tag = "Master Role",
  path = "/api/v1/role/sync-user",
  request_body = SyncRoleToUser,
  responses(
    (status = 200, description = "OK", body = [RoleOAS]),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  security(("token" = [])),
)]
#[put("/sync-user", data = "<request>")]
pub async fn sync_role_user(authentication: Authentication, request: Json<SyncRoleToUser>) -> JsonResponse<Vec<Role>> {
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

/// Sync permission with role
#[utoipa::path(
  put,
  tag = "Master Role",
  path = "/api/v1/role/sync-role",
  request_body = SyncPermissionToRole,
  responses(
    (status = 200, description = "OK", body = [RoleOAS]),
    (status = 400, description = "BAD REQUEST"),
    (status = 500, description = "INTERNAL SERVER ERROR")
  ),
  security(("token" = [])),
)]
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
  routes![all, store, show, update, delete, sync_role_user, sync_permission_role]
}
