use rocket::{Catcher, serde::json::Json};

use crate::responses::Response;

#[catch(401)]
pub async fn unauthorized() -> Json<Response> {
  Json(Response {
    message: "UNAUTHORIZED".to_string()
  })
}

pub fn catchers() -> Vec<Catcher> {
  catchers![unauthorized]
}