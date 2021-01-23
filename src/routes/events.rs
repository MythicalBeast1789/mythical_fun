use rocket_contrib::templates::Template;
use rocket::*;
use crate::routes::structs::TemplateContext;

// parse context
use serde::{Serialize};

// import custom structs for rocket data
use super::structs::*;

#[rocket::get("/")]
pub fn list_events() -> Template {
    let mut context:TemplateContext = TemplateContext{
        page: "2021 events".to_string(),
        message: "".to_string(),
        ok: true
    };

    // event type from route stucks not db!
    let mut events:Vec<Event> = Vec::new();

    match crate::db::list_events() {
        // event type below is different then EVENT in namespace above!
        Ok(evts) => {
            for e in evts {
                let event = Event {
                    title: e.title.to_string(),
                    date: e.date.to_string(),
                    body: e.body.to_string()
                };
                events.push(event)
            }

        }
        Err(e) => {
            use diesel::result::Error;
             match e {
                 Error::NotFound => {
                     context.message = "User/User's events not found!".to_string();
                     context.ok = false
                 },
                 all_other => {
                     println!("{}", all_other);
                     context.message = "Unexpected error happened!".to_string();
                     context.ok = false
                 }
             }
        }
    };

    let context:EvtRes = EvtRes{ context,events };

    Template::render("events/list", &context)
}