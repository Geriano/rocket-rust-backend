use chrono::{
  NaiveDateTime,
  Utc
};
use diesel::{
  prelude::*,
  result::Error,
  PgConnection,
  RunQueryDsl
};
use serde::{
  Deserialize,
  Serialize
};
use uuid::Uuid;

use crate::{
  models::User,
  schemas::api_tokens
};

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable, Identifiable, Selectable, PartialEq, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = api_tokens)]
pub struct ApiToken {
  pub id: String,
  pub user_id: String,
  pub expired_at: Option<NaiveDateTime>,
  pub created_at: NaiveDateTime,
}

impl ApiToken {
  pub fn get_user(conn: &mut PgConnection, id: String) -> Result<User, Error> {
    let api_token: Result<ApiToken, Error> = api_tokens::table
      .filter(api_tokens::id.eq(id))
      .get_result(conn);

    if api_token.is_err() {
      return Err(api_token.err().unwrap())
    }

    let api_token = api_token.unwrap();
    User::find(conn, api_token.user_id)
  }

  pub fn find(conn: &mut PgConnection, id: String) -> Result<ApiToken, Error> {
    api_tokens::table
      .filter(api_tokens::id.eq(id))
      .get_result(conn)
  }

  pub fn create(conn: &mut PgConnection, user_id: String, expired_at: Option<NaiveDateTime>) -> String {
    ApiToken::delete_by_user(conn, user_id.clone()).ok();

    let id = Uuid::new_v4().to_string();
    let api_token = ApiToken {
      id,
      user_id,
      expired_at,
      created_at: Utc::now().naive_local(),
    };

    let api_token = api_token.insert(conn).expect("can't create token");

    api_token.id
  }

  pub fn insert(&self, conn: &mut PgConnection) -> Result<ApiToken, Error> {
    diesel::insert_into(api_tokens::table)
      .values(self)
      .get_result(conn)
  }

  pub fn delete(conn: &mut PgConnection, id: String) -> Result<usize, Error> {
    diesel::delete(api_tokens::table)
      .filter(api_tokens::id.eq(id))
      .execute(conn)
  }

  pub fn delete_by_user(conn: &mut PgConnection, user_id: String) -> Result<usize, Error> {
    diesel::delete(api_tokens::table)
      .filter(api_tokens::user_id.eq(user_id))
      .execute(conn)
  }
}
