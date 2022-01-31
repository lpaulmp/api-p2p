#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use routes::*;

mod db;
mod models;
mod routes;
mod schema;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(db_url);
    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![get_all, new_message, find_username],)
}

fn main() {
    // rocket::ignite().mount("/", routes![hello]).launch();

    rocket().launch();
}