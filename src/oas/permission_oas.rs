use serde::{Deserialize, Serialize};

use crate::{models::Permission};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PermissionOAS {
  pub id: String,
  pub code: String,
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
