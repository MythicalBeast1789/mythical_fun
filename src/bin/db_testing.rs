use mythical_fun::db::list_events;
use mythical_fun::db::models::Event;
use diesel::result::Error;

fn main() {
    let events = list_events();

    match events {
        Ok(evts) => {
            // parse events for using in template
        }
        Err(e) => {
        }
    }
}