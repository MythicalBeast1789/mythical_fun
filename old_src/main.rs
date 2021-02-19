#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
extern crate serde_json;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, QueryResult};
use rocket::request::Form;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use chrono::{NaiveDate};
use dotenv;
use std::env;
use diesel::expression::dsl::date;


#[get("/")]
fn index() -> Template {
    let context:TemplateContext = TemplateContext{page: "home".to_string(), ok: true, message: "".to_string() };
    Template::render("index", &context)
}


#[get("/2021")]
fn twenty_one() -> Template {
    use fun_mythicalcloud_xyz::schema::events::dsl::*;
    let connection = establish_connection();
    let res = events
        .order(date.desc())
        .load::<Event>(&connection)
        .expect("Error loading posts");

    let conxt:TemplateContext = TemplateContext {
        page: "2021 events".to_string(),
        ok: true,
        message: "".to_string()
    };
    let mut v_events:Vec<TempEvent> = Vec::new();

    for e in res  {
        let event = TempEvent {
            date: e.date.format("%a, %b %e").to_string(),
            title: e.title.to_string(),
            body: e.body.to_string()
        };
        v_events.push(event)
    }
    let res:EventsContext = EventsContext {
        context:conxt,
        events: v_events
    };
    Template::render("events", &res)
}

#[get("/2021/add")]
fn add_event_g() -> Template {
    let context:TemplateContext = TemplateContext{
        page: "Add Event ".to_string(),
        ok: false,
        message: "Hi! If you are not Cameron, this page is not for you... sorry!".to_string()
    };

    Template::render("add_event", &context)
}

#[post("/2021/add", data="<event>")]
fn add_event_p(event: Form<structs::events::NewEventForm>) -> Template {
    dotenv::dotenv().ok();

    let mut context:TemplateContext = TemplateContext{
        page: "2021".to_string(),
        ok: true,
        message: "Added event!".to_string()
    };
    let add_auth_key = env::var("ADD_AUTH_KEY").expect("ERROR FINDING VALUE IN AUTH KEY ");
    if event.auth_key != add_auth_key {
        context.ok = false;
        context.message = "WRONG AUTH KEY. EVENT NOT ADDED.".to_string();
        return Template::render("add_event", &context)

    }
    let date_str = event.date.to_string();
    let p_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d");
    if p_date.is_ok() {
        use fun_mythicalcloud_xyz::models::{Event, NewEvent};
        use fun_mythicalcloud_xyz::schema::events;
        let p_date = p_date.unwrap();
        let new_event:NewEvent = NewEvent{
            title: &event.title.to_string(),
            date: &p_date,
            body: &event.body.to_string()
        };
        let conn = establish_connection();
        let db_res:QueryResult<Event> = diesel::insert_into(events::table)
            .values(&new_event)
            .get_result(&conn);
        if db_res.is_ok() {
            context.ok = true;
            context.message = format!("Successfully added event for: {}", date_str)
        } else {
            context.ok = false;
            context.message = "Failed to add date!".to_string();
        }
    } else {
        context.ok = false;
        context.message = "Failed to parse string!".to_string();
    }
    Template::render("add_event", &context)

}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, twenty_one, add_event_g, add_event_p])
        // add path for static files
        .mount("/static/", StaticFiles::from("static/"))
        // add support for template rendering
        .attach(Template::fairing())
        .launch();
}