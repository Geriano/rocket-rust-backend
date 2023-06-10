#[macro_use] extern crate diesel;
#[macro_use] extern crate dotenv;
#[macro_use] extern crate rocket;

use dotenv::dotenv;

mod controllers;
mod helpers;
mod middleware;
mod models;
mod oas;
mod requests;
mod responses;
mod schemas;

#[rocket::launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/api/v1/auth", controllers::auth::routes())
        .mount("/api/v1/user", controllers::user::routes())
        .mount("/api/v1/permission", controllers::permission::routes())
        .mount("/api/v1/role", controllers::role::routes())
}
