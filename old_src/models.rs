use chrono::NaiveDate;

#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub date: NaiveDate
}

#[derive(Insertable)]
#[table_name="events"]
pub struct NewEvent<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub date: &'a NaiveDate
}

