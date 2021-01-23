#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;
use serde::{Serialize};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use mythical_fun::routes::*;

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
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/2021/", routes![events::list_events])
        .mount("/static/", StaticFiles::from("static/"))
        .mount("/users/", routes![users::signup_g, users::signup_p])
        .attach(Template::fairing())
        .launch();
}