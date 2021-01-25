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
    let user = crate::db::users::find_with_email(&u_data.email);
    match user {
        Ok(u) => {
            if u.len() > 0 {
                context.ok = false;
                context.message = "User already exsits!".to_string();
            } else {
                if u_data.password.len() > 7  && u_data.password.len() < 17 {
                    match crate::db::users::insert(&u_data.first_name, &u_data.last_name, &u_data.email, &u_data.password) {
                        Ok(_) => {
                            context.ok = true;
                            context.message = String::from("Successfully signed you up!");
                        }
                        Err(err) => {
                            println!("Error in user route: {}",err);
                            context.ok= false;
                            context.message = String::from("Hmmm... Sometimes not quite right! Please try again later....")
                        }
                    };

                } else {
                    context.ok = false;
                    context.message = "That password is too short! It must be between 8-16".parse().unwrap()
                }
            }
        }
        Err(err) =>  {
            println!("{}", err);
            context.ok = false;
            context.message = "db error".to_string();
        }
    }
    let context:Context = Context{context};
    Template::render("users/signup", &context)
}