#[derive(Queryable)]
pub struct Events {
    pub event_id: i32,
    pub day_id: i32,
    pub title: String
}