use diesel::*;
use super::establish_connection;
use super::models::{User, NewUser};
use std::env;

pub struct UserData {
    pub(crate) user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    password: String
}

// make sure you use the & operator so we don't take ownership out of scope!
pub fn find_with_email(u_email:&String, conn: &PgConnection) -> Result<User, super::structs::DBError>{
    use super::schema::users::dsl::*;
    match users.filter(email.eq(u_email)).limit(1).first::<User>(conn) {
        Ok(us) => {
            Ok(us)
        },
        Err(e) => {
            use super::structs::DBError;
            use diesel::result::Error;
            match e {
                Error::NotFound => Err(DBError::NoneFound),
                all_other => {
                    eprintln!("Error in db/users {}", all_other);
                    Err(DBError::DieselError(all_other))
                }
            }
        }
    }
}


pub fn verify_user(email:&String, password:&String, conn: &PgConnection) -> Result<User, super::structs::DBError> {
    let user = find_with_email(email, conn);
    match user {
        Ok(user) => {
            if user.id > 0 {
                let hash_pass = &user.password;
                let verify = argon2::verify_encoded(&*hash_pass, password.as_ref()).unwrap();
                if verify {
                    Ok(user)
                } else {
                    Err(super::structs::DBError::InvalidData)
                }

            } else {
                Err(super::structs::DBError::InvalidData)
            }
        }
        Err(e)=> {Err(e)}
    }
}

pub fn insert(first_name:&String, last_name:&String, email:&String, password:&String ) -> Result<usize, diesel::result::Error> {
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
        Ok(user_id) => {
            Ok(user_id)
        }
        Err(err) => {
           Err(err)
        }
    }

}

pub fn find_user(user_id:&i32, conn: &PgConnection) -> Option<self::User> {
    use super::schema::users::dsl::*;
    match users.find(user_id).limit(1).first::<User>(conn) {
        Ok(us) => Option::Some(us),
        Err(_) => Option::None
    }}
