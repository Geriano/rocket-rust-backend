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
  models::Permission,
  models::User,
  schemas::permission_user
};

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable, Identifiable, Selectable, PartialEq, Associations)]
#[diesel(belongs_to(Permission))]
#[diesel(belongs_to(User))]
#[diesel(table_name = permission_user)]
#[diesel(primary_key(permission_id, user_id))]
pub struct PermissionUser {
  pub id: String,
  pub permission_id: String,
  pub user_id: String,
  pub created_at: NaiveDateTime,
}

impl PermissionUser {
  pub fn permission_in(conn: &mut PgConnection, permissions: Vec<String>) -> Result<Vec<PermissionUser>, Error> {
    permission_user::table
      .filter(permission_user::id.eq_any(permissions))
      .get_results(conn)
  }

  pub fn insert_many(conn: &mut PgConnection, permission_user: Vec<PermissionUser>) -> Result<Vec<PermissionUser>, Error> {
    diesel::insert_into(permission_user::table)
      .values(permission_user)
      .get_results(conn)
  }

  pub fn delete_by_user(conn: &mut PgConnection, user_id: String) -> Result<usize, Error> {
    diesel::delete(permission_user::table)
      .filter(permission_user::user_id.eq(user_id))
      .execute(conn)
  }
}
