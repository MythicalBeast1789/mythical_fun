pub mod models;
pub mod schema;
pub mod users;

use diesel::{PgConnection, Connection, RunQueryDsl};
use crate::diesel::query_dsl::*;
use std::env;
use dotenv::dotenv;
use crate::db::models::Event;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    // TODO: fix this.
    let mut database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    /*
    match env::var("ROCKET_ENV") {
        Ok(val) => {
            if val == "dev" || val== "development" {
                database_url = env::var("DEV_DB_URL").expect("ROCKET SET IN DEV MODE BUT DEV_DB_URL NOT SET!");
            }
        }
    } */
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn list_events() -> Result<Vec<Event>, diesel::result::Error>{
    use schema::users::dsl::*;
    use self::models::{User, Event};
    let conn = establish_connection();

    // search for a events belonging to use...
    // Use ? operator for automatic error returning
    let user = users.find(2).first::<User>(&conn)?;
    let evts = Event::belonging_to(&user).load::<Event>(&conn)?;
    Ok(evts)

}