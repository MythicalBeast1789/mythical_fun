pub enum DBError {
    DieselError(diesel::result::Error),
    InvalidData,
    NoneFound,
    InvalidUser,
    InvalidUserCredentials,
    Other(String)
}

