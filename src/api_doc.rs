use utoipa::Modify;
use utoipa::openapi::{ComponentsBuilder, schema};
use utoipa::openapi::security::{SecurityScheme, HttpBuilder, HttpAuthScheme};
use utoipa::{OpenApi, openapi};

use crate::{controllers, schemas};
use crate::oas;
use crate::requests::{self, LoginRequest};
use crate::responses;

#[derive(OpenApi)]
#[openapi(
  modifiers(&Authentication),
  paths(
    controllers::auth::login,
    controllers::auth::user,
    controllers::auth::logout,

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

    schemas(requests::LoginRequest),
    schemas(responses::AuthenticatedResponse),

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
    (name = "Authentication", description = ""),
    (name = "Master User", description = "Master User Resource Endpoint"),
    (name = "Master Permission", description = "Master Permission Resource Endpoint"),
    (name = "Master Role", description = "Master Role Resource Endpoint"),
  ),
)]
pub struct ApiDoc;

struct Authentication;

impl Modify for Authentication {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    let components = openapi.components.as_mut().unwrap();

    components.add_security_scheme(
      "token",
      SecurityScheme::Http(
        HttpBuilder::new()
          .scheme(HttpAuthScheme::Bearer)
          .bearer_format("UUID")
          .build(),
      ),
    );
  }
}
