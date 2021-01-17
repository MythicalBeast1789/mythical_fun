#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::{env};

use diesel::prelude::*;
use dotenv::dotenv;
use crate::models::NewEvent;
use crate::models::Event;
use chrono::NaiveDate;

pub mod models;
pub mod schema;

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

pub fn create_event(conn: &PgConnection, title: &str, body: &str, date_str: &str) -> Event{
    use schema::events;
    let date:NaiveDate = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("Failed to parse date");

    let new_event = NewEvent{title, body, date: &date };
    diesel::insert_into(events::table)
        .values(&new_event)
        .get_result(conn).expect("Error adding to DB")
}