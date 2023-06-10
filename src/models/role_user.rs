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
  models::Role,
  models::User,
  schemas::role_user
};

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable, Identifiable, Selectable, PartialEq, Associations)]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(User))]
#[diesel(table_name = role_user)]
#[diesel(primary_key(role_id, user_id))]
pub struct RoleUser {
  pub id: String,
  pub role_id: String,
  pub user_id: String,
  pub created_at: NaiveDateTime,
}

impl RoleUser {
  pub fn role_in(conn: &mut PgConnection, roles: Vec<String>) -> Result<Vec<RoleUser>, Error> {
    role_user::table
      .filter(role_user::id.eq_any(roles))
      .get_results(conn)
  }

  pub fn insert_many(conn: &mut PgConnection, role_user: Vec<RoleUser>) -> Result<Vec<RoleUser>, Error> {
    diesel::insert_into(role_user::table)
      .values(role_user)
      .get_results(conn)
  }

  pub fn delete_by_user(conn: &mut PgConnection, user_id: String) -> Result<usize, Error> {
    diesel::delete(role_user::table)
      .filter(role_user::user_id.eq(user_id))
      .execute(conn)
  }
}
