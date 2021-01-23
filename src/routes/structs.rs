// this file containes structs only for use in ROUTES crate!

use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Event {
    pub title: String,
    pub date: String,
    pub body: String
}

#[derive(Serialize)]
pub struct EvtRes {
    pub context:TemplateContext,
    pub events: Vec<Event>
}

#[derive(Serialize)]
pub struct Context {
    pub context:TemplateContext
}

#[derive(Serialize)]
pub struct TemplateContext {
    pub page: String,
    pub message: String,
    pub ok: bool
}

#[derive(rocket::FromForm)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String
}