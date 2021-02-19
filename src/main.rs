#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_json;
// #[macro_use] extern crate diesel;
use serde::{Serialize};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use mythical_fun::routes::*;

use rocket_contrib::databases::diesel;
use argon2::Config;
use mythical_fun::DBConn;


// #[database("database")]
// pub struct DBConn(diesel::PgConnection);

#[get("/")]
pub fn index() -> Template {
    #[derive(Serialize)]
    struct Tcontext {
        ok: bool
    }
    let context = Tcontext{
        ok: true
    };
    Template::render("index", &context)
}
fn main() {
    use rocket::config::{Config};
    use dotenv::dotenv;
    dotenv().ok();
    let event_routes = routes![events::list_events, events::add_g, events::add_p];

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/2021/", event_routes)
        .mount("/static/", StaticFiles::from("static/"))
        .mount("/users/", routes![
            users::signup_g,
            users::signup_p,
            users::login_g,
            users::login_p])
        .attach(Template::fairing())
        .attach(DBConn::fairing())
        .launch();
}