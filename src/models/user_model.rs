use chrono::{
  NaiveDateTime,
  Utc
};
use diesel::{
  prelude::*,
  result::Error,
  Queryable,
  Insertable,
  PgConnection
};
use serde::{
  Serialize,
  Deserialize
};

use crate::{
  models::Permission,
  models::Role,
  models::RoleUser,
  schemas::{
    permissions,
    roles,
    users,
  },
};

use super::PermissionUser;

#[derive(Clone, Debug, Insertable, Queryable, Serialize, Deserialize, Identifiable, Selectable, PartialEq)]
pub struct User {
  pub id: String,
  pub name: String,
  pub email: String,
  pub email_verified_at: Option<NaiveDateTime>,
  pub username: String,
  pub password: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub deleted_at: Option<NaiveDateTime>,
}

impl User {
  pub fn find(conn: &mut PgConnection, id: String) -> Result<User, Error> {
    users::table.filter(users::id.eq(id)).first(conn)
  }

  pub fn find_by_email(conn: &mut PgConnection, email: String) -> Result<User, Error> {
    users::table.filter(users::email.eq(email)).first(conn)
  }

  pub fn find_by_username(conn: &mut PgConnection, username: String) -> Result<User, Error> {
    users::table.filter(users::username.eq(username)).first(conn)
  }

  pub fn insert(&self, conn: &mut PgConnection) -> Result<User, Error> {
    diesel::insert_into(users::table)
      .values(self)
      .get_result(conn)
  }

  pub fn update(&self, conn: &mut PgConnection) -> Result<usize, Error> {
    diesel::update(users::table)
      .filter(users::id.eq(self.id.clone()))
      .set((
        users::name.eq(self.name.clone()),
        users::email.eq(self.email.clone()),
        users::email_verified_at.eq(self.email_verified_at.clone()),
        users::username.eq(self.username.clone()),
        users::password.eq(self.password.clone()),
        users::updated_at.eq(Utc::now().naive_local()),
      ))
      .execute(conn)
  }

  pub fn delete(&mut self, conn: &mut PgConnection) -> Result<usize, Error> {
    self.deleted_at = Some(Utc::now().naive_local());
    self.update(conn)
  }

  pub fn recovery(&mut self, conn: &mut PgConnection) -> Result<usize, Error> {
    self.deleted_at = None;
    self.update(conn)
  }

  pub fn purge(&self, conn: &mut PgConnection) -> Result<usize, Error> {
    diesel::delete(users::table)
      .filter(users::id.eq(self.id.clone()))
      .execute(conn)
  }

  pub fn get_permissions(&self, conn: &mut PgConnection) -> Result<Vec<Permission>, Error> {
    PermissionUser::belonging_to(self)
      .inner_join(permissions::table)
      .select(Permission::as_select())
      .load(conn)
  }

  pub fn get_roles(&self, conn: &mut PgConnection) -> Result<Vec<Role>, Error> {
    RoleUser::belonging_to(self)
      .inner_join(roles::table)
      .select(Role::as_select())
      .load(conn)
  }
}
