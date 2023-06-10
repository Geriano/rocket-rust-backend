use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PermissionCreateRequest {
  pub code: String,
  pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PermissionUpdateRequest {
  pub name: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncPermissionToUser {
  pub permissions: Vec<String>,
  pub user_id: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncPermissionToRole {
  pub permissions: Vec<String>,
  pub role_id: String,
}
