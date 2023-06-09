#[macro_use] extern crate diesel;
#[macro_use] extern crate dotenv;
#[macro_use] extern crate rocket;

use dotenv::dotenv;

mod controllers;
mod helpers;
mod middleware;
mod models;
mod requests;
mod responses;
mod schemas;

#[rocket::launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
}
