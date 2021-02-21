// this file containes structs only for use in ROUTES crate!

use serde::{Serialize, };
use rocket::request::{FromRequest, Outcome};
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket::{Request};
use crate::DBConn;

#[derive(Debug, rocket::response::Responder)]
pub enum TempOrRedirect{Template(Template), Redirect(Redirect)}

#[derive(rocket::FromForm)]
pub struct NewEvent {
    pub event_date: String,
    pub event_title: String,
    pub event_body: String
}

#[derive(Serialize)]
pub struct Event {
    pub title: String,
    pub date: String,
    pub body: String
}

#[derive(Serialize)]
pub struct EvtRes {
    pub context:TemplateContext,
    pub users_name: String,
    pub can_edit: bool,
    pub events: Vec<Event>
}

pub enum Payload{
    User,
    Other(String),
    None
}

#[derive(Serialize)]
pub struct Context {
    pub context:TemplateContext,
}

#[derive(Serialize)]
pub struct TemplateContext {
    pub page: String,
    pub message: String,
    pub ok: bool,
    pub logged_in: bool
}

#[derive(rocket::FromForm)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String
}

pub struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
    email: String
}
pub struct RequestContext{
    pub method: rocket::http::Method,
    pub route: String,
    pub user: Option<crate::db::models::User>,
    pub template_context: TemplateContext,
}

impl<'a, 'r> FromRequest<'a ,'r> for RequestContext {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<RequestContext, Self::Error> {
        let user_id = request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok());

        let conn = request.guard::<DBConn>()?;
        let user = if user_id.is_some() {crate::db::users::find_user(&user_id.unwrap(), &*conn)} else {Option::None};
        let user_loggedin = if user.is_some() {true} else { false };

        Outcome::Success(RequestContext {
            user: user,
            method: request.method(),
            route: request.uri().path().to_string(),
            template_context: TemplateContext {
                page: String::from(""),
                message: "".to_string(),
                ok: false,
                logged_in: user_loggedin,
            }
        })
    }
}