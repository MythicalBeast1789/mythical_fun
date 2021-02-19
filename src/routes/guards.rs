use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

struct AuthKey(String);
//
// #[derive(Debug)]
// enum AuthKeyError(
// Missing
// Invalid
// )