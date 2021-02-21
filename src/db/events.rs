use super::structs::DBError;
use super::models::{NewEvent, Event};
use chrono::NaiveDate;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::query_dsl::*;

pub fn add_event(user_id:&i32, date:&String, title: &String, body:&String) -> Result<Event, DBError> {
    use super::schema::events;
    let date = NaiveDate::from(date.parse().unwrap());
    let new_event = NewEvent {
        user_id, title,body, date: &date};
    let conn = super::establish_connection();
    match diesel::insert_into(events::table)
        .values(&new_event)
        .get_result(&conn) {
        Ok(evt) => {
            Ok(evt)
        },
        Err(err) => {
            println!("Error adding event {}",err);
         Err(DBError::DieselError(err))

        }
    }
}

pub fn list_events(stored_user_id: &i32) -> Result<Vec<Event>, DBError> {
    use super::models::{User, Event};
    use super::schema::users::dsl::*;
    use super::schema::events;
    let conn = super::establish_connection();
    let user = users.filter(id.eq(stored_user_id)).limit(1).first::<User>(&conn);
    use diesel::result::Error;
    match user {
        Ok(u) => {
            match Event::belonging_to(&u).order(events::date.desc()).load::<Event>(&conn) {
                Ok(evts) => {Ok(evts)},
                Err(e) => {
                    match e {
                        Error::NotFound => {Err(DBError::NoneFound)},
                        other => {Err(DBError::DieselError(other))}
                    }
                }
            }
        },
        Err(e) => {
            match e {
                Error::NotFound => Err(DBError::InvalidUser),
                other_errors => Err(DBError::DieselError(other_errors))
            }
        }
    }

}
