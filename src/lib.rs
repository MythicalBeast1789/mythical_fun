#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
extern crate serde_json;
pub mod routes;
pub mod db;

use rocket_contrib::databases::diesel::*;
use rocket_contrib::database;

#[database("database")]
pub struct DBConn(diesel::PgConnection);

