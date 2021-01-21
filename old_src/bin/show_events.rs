extern crate diesel;

use diesel::prelude::*;

use fun_mythicalcloud_xyz::establish_connection;
use fun_mythicalcloud_xyz::models::Event;

fn main() {
    use fun_mythicalcloud_xyz::schema::events::dsl::*;

    let connection = establish_connection();
    let results = events
        .limit(5)
        .load::<Event>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
    }
}