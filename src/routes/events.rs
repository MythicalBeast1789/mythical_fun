use rocket_contrib::templates::Template;
use crate::routes::structs::TemplateContext;
use rocket::*;


#[rocket::get("/")]
pub fn list_events() -> Template {
    let context:TemplateContext = TemplateContext{
        page: "Index".to_string(),
        message: "Hi".to_string(),
        ok: true
    };
    Template::render("index", &context)
}