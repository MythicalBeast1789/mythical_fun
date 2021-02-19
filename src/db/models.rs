use chrono::{NaiveDate};
use super::schema::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub share_code: String,
    // TODO: add joined at column...
}


#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[table_name="events"]
pub struct Event {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub date: NaiveDate
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub password: &'a str
}

#[derive(Insertable, Associations)]
#[belongs_to(User)]
#[table_name="events"]
pub struct NewEvent<'a> {
    pub user_id: &'a i32,
    pub title: &'a str,
    pub body: &'a str,
    pub date: &'a NaiveDate
}

