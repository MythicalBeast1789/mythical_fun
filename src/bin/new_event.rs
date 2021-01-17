extern crate diesel;

use diesel::prelude::*;

use fun_mythicalcloud_xyz::{establish_connection, create_event};
use fun_mythicalcloud_xyz::models::{ NewEvent};
use chrono::NaiveDate;
use std::str::FromStr;

fn main() {
    use fun_mythicalcloud_xyz::schema::events;
    let date:NaiveDate = NaiveDate::parse_from_str("2021-01-02", "%Y-%m-%d").expect("error parsing date");
    let connection = establish_connection();
    let new_event = NewEvent{
        title: "hi",
        body: "hi",
        date: &date
    };
    create_event(&connection, &"hi", &"hi", &"2021-01-02");

}