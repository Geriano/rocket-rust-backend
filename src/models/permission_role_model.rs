use chrono::NaiveDateTime;
use diesel::{
  prelude::*,
  result::Error,
  PgConnection
};
use serde::{
  Deserialize,
  Serialize
};

use crate::{
  models::{
    Permission,
    Role
  },
  schemas::permission_role
};

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable, Identifiable, Selectable, PartialEq, Associations)]
#[diesel(belongs_to(Permission))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = permission_role)]
#[diesel(primary_key(permission_id, role_id))]
pub struct PermissionRole {
  pub id: String,
  pub permission_id: String,
  pub role_id: String,
  pub created_at: NaiveDateTime,
}

impl PermissionRole {
  pub fn permission_role_in(conn: &mut PgConnection, permissions: Vec<String>) -> Result<Vec<PermissionRole>, Error> {
    permission_role::table
      .filter(permission_role::id.eq_any(permissions))
      .get_results(conn)
  }

  pub fn insert_many(conn: &mut PgConnection, permission_role: Vec<PermissionRole>) -> Result<Vec<PermissionRole>, Error> {
    diesel::insert_into(permission_role::table)
      .values(permission_role)
      .get_results(conn)
  }

  pub fn delete_by_role(conn: &mut PgConnection, role_id: String) -> Result<usize, Error> {
    diesel::delete(permission_role::table)
      .filter(permission_role::role_id.eq(role_id))
      .execute(conn)
  }
}
