extern crate serde_json;
use serde::{Serialize};

#[derive(Serialize, FromForm)]
pub struct NewEventForm {
    pub title: String,
    pub body: String,
    pub date: String,
    pub auth_key: String
}

#[derive(Serialize)]
pub struct TempEvent {
    pub date: String,
    pub title: String,
    pub body: String
}
#[derive(Serialize)]
pub struct EventsContext {
    pub context: TemplateContext,
    pub events: Vec<TempEvent>
}

#[derive(Serialize)]
pub struct TemplateContext {
    pub page: String,
    pub ok: bool,
    pub message: String,
}