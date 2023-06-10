use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoleCreateRequest {
  pub code: String,
  pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoleUpdateRequest {
  pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncRoleToUser {
  pub roles: Vec<String>,
  pub user_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncPermissionToRole {
  pub roles: Vec<String>,
  pub role_id: String,
}
