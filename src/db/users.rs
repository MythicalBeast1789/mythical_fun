use diesel::*;
use super::establish_connection;
use super::models::User;


pub fn find_with_email(u_email:String) -> Result<Vec<User>, diesel::result::Error>{
    use super::schema::users::dsl::*;
    use super::models::User;
    let conn = establish_connection();
    let user = users.filter(email.eq(u_email)).load::<User>(&conn)?;
    Ok(user)
}

pub fn insert()
