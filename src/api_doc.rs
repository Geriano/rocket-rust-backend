use utoipa::{OpenApi, openapi};

use crate::controllers;
use crate::oas;
use crate::requests;

#[derive(OpenApi)]
#[openapi(
    paths(
      controllers::user::pagination,
      controllers::user::store,
      controllers::user::show,
      controllers::user::update,
      controllers::user::update_password,
      controllers::user::delete,
      controllers::user::purge,
    ),
    components(
      schemas(oas::UserOAS),
      schemas(oas::UserPaginationOAS),
      schemas(requests::UserStoreRequest),
      schemas(requests::UserUpdateGeneralInformationRequest),
      schemas(requests::UserUpdatePasswordRequest),
    ),
    tags(
      (name = "Master User", description = "Master User Resource Endpoint")
    ),
)]
pub struct ApiDoc;