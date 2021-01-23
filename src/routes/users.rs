use crate::routes::structs::*;
use rocket_contrib::templates::Template;
use rocket::request::Form;


#[rocket::get("/signup")]
pub fn signup_g() -> Template {
    let context:TemplateContext = TemplateContext {
        page: "Signup".to_string(),
        message: "".to_string(),
        ok: true
    };
    let context:Context = Context{ context };
    Template::render("users/signup", &context)
}

#[rocket::post("/signup", data="<u_data>")]
pub fn signup_p(u_data:Form<NewUser>) -> Template{
    let mut context:TemplateContext = TemplateContext{
        page: "Signup result".to_string(),
        message: "Success".to_string(),
        ok: true
    };
    let user = crate::db::users::find_with_email(u_data.into_inner().email);
    match user {
        Ok(u) => {
            if u.len() > 0 {
                context.ok = false;
                context.message = "User already exsits!".to_string();
            } else {
                if u_data.password.len() > 7  && u_data.password.len() < 17 {


                } else {
                    contest.message = "That password is too short! It must be between 8-16"
                }
            }
        }
        Err(err) =>  {
            match err {
                diesel::NotFound => {
                    context.message = "Uesr not found in db. REay to add".to_string()
                    // TODO: add function to add user to db
                }
                other_error => {
                    println!("Some other error occured: {}", other_error)
                }
            }
        }
    }
    let context:Context = Context{context};
    Template::render("users/signup", &context)
}