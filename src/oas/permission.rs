use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, openapi::schema};

use crate::{models::Permission};

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct PermissionOAS {
  #[schema(example = "f6943706-7a06-4b62-840d-05327e6c6b3b")]
  pub id: String,

  #[schema(example = "TEST")]
  pub code: String,

  #[schema(example = "Test")]
  pub name: String,
}

impl From<Permission> for PermissionOAS {
  fn from(permission: Permission) -> Self {
    Self {
      id: permission.id,
      code: permission.code,
      name: permission.name,
    }
  }
}
