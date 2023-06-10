use chrono::{
  NaiveDateTime,
  Utc
};
use diesel::{
  prelude::*,
  result::Error
};
use serde::{
  Deserialize,
  Serialize
};

use crate::{
  models::Permission,
  schemas::{roles, permissions}
};

use super::PermissionRole;

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable, Identifiable, Selectable, PartialEq)]
pub struct Role {
  pub id: String,
  pub code: String,
  pub name: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl Role {
  pub fn all(conn: &mut PgConnection) -> Result<Vec<Role>, Error> {
    roles::table
      .get_results(conn)
  }
  
  pub fn find(conn: &mut PgConnection, id: String) -> Result<Role, Error> {
    roles::table
      .filter(roles::id.eq(id))
      .get_result(conn)
  }

  pub fn find_by_code(conn: &mut PgConnection, code: String) -> Result<Role, Error> {
    roles::table
      .filter(roles::code.eq(code))
      .get_result(conn)
  }

  pub fn id_in(conn: &mut PgConnection, id: Vec<String>) -> Result<Vec<Role>, Error> {
    roles::table
      .filter(roles::id.eq_any(id))
      .get_results(conn)
  }

  pub fn insert(&self, conn: &mut PgConnection) -> Result<Role, Error> {
    diesel::insert_into(roles::table)
      .values(self)
      .get_result(conn)
  }

  pub fn update(&self, conn: &mut PgConnection) -> Result<Role, Error> {
    diesel::update(roles::table)
      .set((
        roles::code.eq(self.code.clone()),
        roles::name.eq(self.name.clone()),
        roles::updated_at.eq(Utc::now().naive_local()),
      ))
      .get_result(conn)
  }

  pub fn delete(&self, conn: &mut PgConnection) -> Result<usize, Error> {
    diesel::delete(roles::table)
      .filter(roles::id.eq(self.id.clone()))
      .execute(conn)
  }

  pub fn get_permissions(&self, conn: &mut PgConnection) -> Result<Vec<Permission>, Error> {
    PermissionRole::belonging_to(self)
      .inner_join(permissions::table)
      .select(Permission::as_select())
      .load(conn)
  }
}
