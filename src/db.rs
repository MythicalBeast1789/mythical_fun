mod models;
mod schema;

use diesel::{PgConnection, Connection};
use std::env;
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let mut database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match env::var("ROCKET_ENV") {
        Ok(val) => {if val == "dev" || val== "development" {
            database_url = env::var("DEV_DB_URL").expect("ROCKET SET IN DEV MODE BUT DEV_DB_URL NOT SET!");
        }}
        Err(_) => {},
    }
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}