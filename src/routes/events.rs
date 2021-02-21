use rocket_contrib::templates::Template;
use crate::routes::structs::{TemplateContext, TempOrRedirect};
use super::structs::*;
use rocket::request::Form;
use rocket::response::Redirect;

#[rocket::post("/add", data="<event_data>")]
pub fn add_p(event_data:Form<NewEvent>, request: RequestContext) -> TempOrRedirect {

    let mut context = Context {
        context: TemplateContext {
            page: "event added".to_string(),
            message: "".to_string(),
            ok: false,
            logged_in: false
        }
    };
    if request.user.is_some() {
        match crate::db::events::add_event(&request.user.unwrap().id, &event_data.event_date, &event_data.event_title, &event_data.event_body) {
            Ok(evt) => {
                println!("Event date: {}", evt.title);
                context.context.ok = true;
                context.context.message = String::from("Added event");
                TempOrRedirect::Template(Template::render("events/add", &context))
            },
            Err(_) => {
                context.context.message = String::from("Error adding event!");
                TempOrRedirect::Template(Template::render("events/add", &context))
            }
        }
    } else {
        context.context.message = String::from("Please login to use this feature!");
        TempOrRedirect::Redirect(Redirect::to("/users/login"))
    }
}

#[rocket::get("/add")]
pub fn add_g(request:RequestContext) -> TempOrRedirect {

    let mut context = Context{ context: TemplateContext {
        page: "Add event".to_string(),
        message: "".to_string(),
        ok: false,
        logged_in: false
    }};
    if request.user.is_some() {
        context.context.ok = true;
        TempOrRedirect::Template(Template::render("events/add", &context))
    } else {
        use rocket::response::Redirect;

        TempOrRedirect::Redirect(Redirect::to("/users/login"))
    }

}

#[rocket::get("/")]
pub fn list_events(request: RequestContext) -> Template {
    let mut context:TemplateContext = TemplateContext{
        page: "2021 events".to_string(),
        message: "".to_string(),
        ok: true,
        logged_in: false
    };
    let mut user_id = 1;
    let mut users_name = "Cameron".to_string();

    if request.user.is_some() {
        let user = request.user.unwrap();
        user_id = *&user.id;
        users_name = user.first_name;
    }
    let can_edit = if request.template_context.logged_in{true} else {false};
    // event type from route stucks not db!
    let mut events:Vec<Event> = Vec::new();

    match crate::db::events::list_events(&user_id) {
        // event type below is different then EVENT in namespace above!
        Ok(evts) => {
            for e in evts {
                let event = Event {
                    title: e.title.to_string(),
                    date: e.date.format("%m/%d/%Y").to_string(),
                    body: e.body.to_string()
                };
                events.push(event)
            }

        }
        Err(e) => {
            use crate::db::structs::DBError;
            match e {
                DBError::NoneFound => {
                    context.message = "No Events found!".to_string();
                }
                DBError::InvalidUser => context.message = String::from("No user found!"),
                DBError::DieselError(db) => {println!("database error: {}", db)},
                _all_other => println!("Something else happened")
            }
        }
    };

    let context:EvtRes = EvtRes{ context,events, users_name, can_edit};

    Template::render("events/list", &context)
}
