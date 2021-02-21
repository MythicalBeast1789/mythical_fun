pub mod models;
pub mod schema;
pub mod users;
pub mod structs;
pub mod events;

use diesel::{PgConnection, Connection, RunQueryDsl};
use crate::diesel::query_dsl::*;
use std::env;
use dotenv::dotenv;
use crate::db::models::Event;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let mut database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
