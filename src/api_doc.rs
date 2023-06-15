use utoipa::{OpenApi, openapi};

use crate::controllers;
use crate::oas;
use crate::requests;

#[derive(OpenApi)]
#[openapi(
  paths(
    controllers::user::pagination,
    controllers::user::store,
    controllers::user::show,
    controllers::user::update,
    controllers::user::update_password,
    controllers::user::delete,
    controllers::user::purge,

    controllers::permission::all,
    controllers::permission::store,
    controllers::permission::show,
    controllers::permission::update,
    controllers::permission::delete,
    controllers::permission::sync_permission_user,
    controllers::permission::sync_permission_role,

    controllers::role::all,
    controllers::role::store,
    controllers::role::show,
    controllers::role::update,
    controllers::role::delete,
    controllers::role::sync_role_user,
    controllers::role::sync_permission_role,
  ),
  components(
    schemas(oas::UserOAS),
    schemas(oas::UserPaginationOAS),
    schemas(oas::PermissionOAS),
    schemas(oas::RoleOAS),

    schemas(requests::PaginationRequest),

    schemas(requests::UserStoreRequest),
    schemas(requests::UserUpdateGeneralInformationRequest),
    schemas(requests::UserUpdatePasswordRequest),

    schemas(requests::PermissionCreateRequest),
    schemas(requests::PermissionUpdateRequest),
    schemas(requests::SyncPermissionToUser),
    schemas(requests::SyncPermissionToRole),

    schemas(requests::RoleCreateRequest),
    schemas(requests::RoleUpdateRequest),
    schemas(requests::SyncRoleToUser),
    schemas(requests::SyncPermissionToRole),
  ),
  tags(
    (name = "Master User", description = "Master User Resource Endpoint"),
    (name = "Master Permission", description = "Master Permission Resource Endpoint"),
    (name = "Master Role", description = "Master Role Resource Endpoint"),
  ),
)]
pub struct ApiDoc;