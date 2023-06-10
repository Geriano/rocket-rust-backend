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
  schemas::permissions
};

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable, Identifiable, Selectable, PartialEq)]
pub struct Permission {
  pub id: String,
  pub code: String,
  pub name: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl Permission {
  pub fn all(conn: &mut PgConnection) -> Result<Vec<Permission>, Error> {
    permissions::table
      .get_results(conn)
  }
  
  pub fn find(conn: &mut PgConnection, id: String) -> Result<Permission, Error> {
    permissions::table
      .filter(permissions::id.eq(id))
      .get_result(conn)
  }

  pub fn find_by_code(conn: &mut PgConnection, code: String) -> Result<Permission, Error> {
    permissions::table
      .filter(permissions::code.eq(code))
      .get_result(conn)
  }

  pub fn id_in(conn: &mut PgConnection, id: Vec<String>) -> Result<Vec<Permission>, Error> {
    permissions::table
      .filter(permissions::id.eq_any(id))
      .get_results(conn)
  }

  pub fn insert(&self, conn: &mut PgConnection) -> Result<Permission, Error> {
    diesel::insert_into(permissions::table)
      .values(self)
      .get_result(conn)
  }

  pub fn update(&self, conn: &mut PgConnection) -> Result<Permission, Error> {
    diesel::update(permissions::table)
      .set((
        permissions::code.eq(self.code.clone()),
        permissions::name.eq(self.name.clone()),
        permissions::updated_at.eq(Utc::now().naive_local()),
      ))
      .get_result(conn)
  }

  pub fn delete(&self, conn: &mut PgConnection) -> Result<usize, Error> {
    diesel::delete(permissions::table)
      .filter(permissions::id.eq(self.id.clone()))
      .execute(conn)
  }
}
