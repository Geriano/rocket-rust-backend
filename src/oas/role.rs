use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, openapi::schema};

use crate::{models::Role};

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct RoleOAS {
  #[schema(example = "f6943706-7a06-4b62-840d-05327e6c6b3b")]
  pub id: String,

  #[schema(example = "TEST")]
  pub code: String,

  #[schema(example = "Test")]
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
