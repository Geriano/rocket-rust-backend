use serde::{Deserialize, Serialize};

use crate::{models::Role};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoleOAS {
  pub id: String,
  pub code: String,
  pub name: String,
}

impl From<Role> for RoleOAS {
  fn from(role: Role) -> Self {
    Self {
      id: role.id,
      code: role.code,
      name: role.name,
    }
  }
}
