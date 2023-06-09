use rocket::{
  request::{
    FromRequest,
    self
  },
  Request,
  outcome::Outcome,
  http::Status
};
use serde::{
  Deserialize,
  Serialize
};

use crate::{
  models::{
    User,
    ApiToken,
    Permission,
    Role
  }, helpers::get_conn,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Authentication {
  pub token: String,
  pub user: User,
  pub permissions: Vec<Permission>,
  pub roles: Vec<Role>,
}

#[derive(Debug)]
pub enum ApiTokenError {
  Missing,
  Invalid,
}

impl<'r> FromRequest<'r> for Authentication {
  type Error = ApiTokenError;

  fn from_request<'life0,'async_trait>(request: &'r Request<'life0>) ->  core::pin::Pin<Box<dyn core::future::Future<Output = request::Outcome<Self,Self::Error> > + core::marker::Send+'async_trait> >where 'r:'async_trait,'life0:'async_trait,Self:'async_trait {
    let token = request.headers().get_one("authorization");
    let outcome = match token {
      Some(token) => {
        let token = token.to_string();

        if !token.starts_with("Bearer") {
          return Box::pin(async move {
            Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid))
          });
        }

        let mut conn = get_conn();
        let token = token.replace("Bearer ", "");
        let api_token = ApiToken::find(&mut conn, token.clone());

        if api_token.is_err() {
          return Box::pin(async move {
            Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid))
          });
        }

        let user = User::find(&mut conn, api_token.unwrap().user_id);

        if user.is_err() {
          return Box::pin(async move {
            Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid))
          });
        }

        let user = user.unwrap();
        let permissions: Vec<Permission> = user.get_permissions(&mut conn).unwrap();
        let roles: Vec<Role> = user.get_roles(&mut conn).unwrap();

        Outcome::Success(Authentication {
          token,
          user,
          permissions,
          roles,
        })
      }
      None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
    };

    Box::pin(async move {
      outcome
    })
  }
}
