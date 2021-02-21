use crate::routes::structs::*;
use rocket_contrib::templates::Template;
use rocket::request::{Form, FromForm};
use rocket::http::{Cookies, Cookie};
use crate::DBConn;
use rocket::response::Redirect;


#[rocket::get("/signup")]
pub fn signup_g() -> Template {
    let context:TemplateContext = TemplateContext {
        page: "Signup".to_string(),
        message: "".to_string(),
        ok: true,
        logged_in: false
    };
    let context:Context = Context{ context };
    Template::render("users/signup", &context)
}

#[rocket::post("/signup", data="<u_data>")]
pub fn signup_p(u_data:Form<NewUser>, db_conn: DBConn) -> Template{
    let mut context:TemplateContext = TemplateContext{
        page: "Signup result".to_string(),
        message: "Success".to_string(),
        ok: true,
        logged_in: false
    };
    // TODO: REWORK THIS SECTION WITH NEW RETURN TYPE OF FUNCTION
    let user = crate::db::users::find_with_email(&u_data.email, &*db_conn);
    use rocket::response::Redirect;
    match user {
        Ok(_) => {
            Redirect::to("/users/login");
        }
        Err(err) =>  {
            use crate::db::structs::DBError;
            match err {
                DBError::NoneFound => {
                    if u_data.password.len() < 17 && u_data.password.len() > 7 {
                        match crate::db::users::insert(&u_data.first_name, &u_data.last_name, &u_data.email, &u_data.password) {
                            Ok(_) => {
                                // TODO: login user who just created account wihtout redirect
                                Redirect::to("/users/login");
                            },
                            Err(e) => {
                                context.ok = false;
                                context.message = String::from("Uh oh... Something went wrong, try again!");
                                println!("{}", e)
                            }
                        }
                    } else {
                        context.ok = false;
                        context.message = String::from("hmm.. Please double check that password! Passwords must be between 8 and 18 characters long!");
                    }
                },
                _all_other => {
                    println!("Error creating user!");
                    context.ok = false;
                    context.message = String::from("Uh oh... Something went wrong... Please try again later...")
                }

            }
        }
    }
    let context:Context = Context{context};
    Template::render("users/signup", &context)
}

#[rocket::get("/login")]
pub fn login_g(request_context: RequestContext) -> Template {
    let context:Context = Context {
        context: TemplateContext {
            page: String::from("Login!"),
            ok: true,
            message: String::from(""),
            logged_in: if request_context.user.is_some() {true} else {false}
        }
    };
    Template::render("users/login", &context)
}

#[derive(FromForm)]
pub struct LoginCreds {
    email: String,
    password: String
}

#[rocket::post("/login", data="<creds>")]
pub fn login_p(creds: Form<LoginCreds>, mut context:RequestContext, conn: DBConn ,mut cookie: Cookies) -> Result<Redirect, Template> {

    // TODO: CHANGE TO WORK WITH NEW RETURN TYPE
    match crate::db::users::verify_user(&creds.email, &creds.password, &*conn) {
        Ok(user) => {
                context.template_context.ok = true;
                cookie.add_private(Cookie::new("user_id", user.id.to_string()));
                return Result::Ok(Redirect::to("/2021/"))        }
        Err(e) => {
            context.template_context.ok = false;
            use crate::db::structs::DBError;
            match e {
                DBError::InvalidData | DBError::NoneFound => {
                    context.template_context.message = String::from("Incorrect username/password!")
                },
                _all_other => {
                    context.template_context.message = String::from("Hmm... Something went wrong! Please try again later!")
                }
            }
        }
    }
    let context:Context = Context{
        context: context.template_context
    };
     Result::Err(Template::render("users/login", &context))
}
