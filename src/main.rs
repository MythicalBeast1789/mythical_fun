#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;
mod lib;
mod db;
mod routes;

use lib::*;
use rocket_contrib::templates::Template;
use routes::events::*;

fn main() {
    rocket::ignite()
        .mount("/2021/", routes![routes::events::list_events])
        .attach(Template::fairing())
        .launch();
}