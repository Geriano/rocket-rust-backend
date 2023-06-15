use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, openapi::schema};


#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct RoleCreateRequest {
  #[schema(example = "TEST")]
  pub code: String,

  #[schema(example = "Test")]
  pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct RoleUpdateRequest {
  #[schema(example = "Test")]
  pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct SyncRoleToUser {
  #[schema(example = json!(["f6943706-7a06-4b62-840d-05327e6c6b3b"]))]
  pub roles: Vec<String>,

  #[schema(example = "f6943706-7a06-4b62-840d-05327e6c6b3b")]
  pub user_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct SyncPermissionToRole {
  #[schema(example = json!(["f6943706-7a06-4b62-840d-05327e6c6b3b"]))]
  pub roles: Vec<String>,

  #[schema(example = "f6943706-7a06-4b62-840d-05327e6c6b3b")]
  pub role_id: String,
}
