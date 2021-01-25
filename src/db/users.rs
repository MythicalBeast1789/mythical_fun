use diesel::*;
use super::establish_connection;
use super::models::{User, NewUser};
use std::env;

// make sure you use the & operator so we don't take ownership out of scope!
pub fn find_with_email(u_email:&String) -> Result<Vec<User>, diesel::result::Error>{
    use super::schema::users::dsl::*;
    let conn = establish_connection();
    let user = users.filter(email.eq(u_email)).load::<User>(&conn)?;
    Ok(user)
}

pub fn insert(first_name:&String, last_name:&String, email:&String, password:&String ) -> Result<bool, diesel::result::Error> {
    // debugger says something about redundent import. Not sure why. Dont delete this line!
    use argon2::{self, Config};
    dotenv::dotenv().ok();

    let pass_salt = env::var("HASH_PASSWORD_SALT").expect("COULD NOT FIND SALT HAS ENV");
    let config = Config::default();
    let password = argon2::hash_encoded(password.as_ref(), pass_salt.as_ref(), &config).unwrap();
    println!("Hasshed password: {}", &password);


    let conn = establish_connection();
    use crate::db::schema::users;
    let new_user:NewUser = NewUser{ first_name, last_name, email, password: &*password };
    match diesel::insert_into(users::table).values(&new_user).execute(&conn)
    {
        Ok(_) => {
            Ok(true)
        }
        Err(err) => {
           Err(err)
        }
    }

}
