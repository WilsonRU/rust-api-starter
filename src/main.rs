#[macro_use] extern crate rocket;
extern crate diesel;

use dotenv::dotenv;
use std::env;
use rocket::routes;


mod adapters;
mod core;
mod user;

use adapters::{database::db::establish_connection, http::error_handling::{businesslogic, internal_error}};


#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "4001".to_string())
        .parse()
        .expect("PORT must be a number");

    rocket::build()
        .manage(establish_connection())
        .mount("/core", routes![
            core::rest::post_login::action,
            core::rest::post_register::action
        ])
        .mount("/user", routes![
            user::rest::get_me::action
        ])
        .register("/", catchers![internal_error, businesslogic])
        .configure(rocket::Config {
            port,
            ..Default::default()
        })
}