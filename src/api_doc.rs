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
  ),
  components(
    schemas(oas::UserOAS),
    schemas(oas::UserPaginationOAS),
    schemas(oas::PermissionOAS),

    schemas(requests::UserStoreRequest),
    schemas(requests::UserUpdateGeneralInformationRequest),
    schemas(requests::UserUpdatePasswordRequest),
    schemas(requests::PermissionCreateRequest),
    schemas(requests::PermissionUpdateRequest),
    schemas(requests::SyncPermissionToUser),
    schemas(requests::SyncPermissionToRole),
  ),
  tags(
    (name = "Master User", description = "Master User Resource Endpoint"),
    (name = "Master Permission", description = "Master Permission Resource Endpoint"),
  ),
)]
pub struct ApiDoc;