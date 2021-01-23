table! {
    events (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        body -> Text,
        date -> Date,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        share_code -> Varchar,
    }
}

joinable!(events -> users (user_id));

allow_tables_to_appear_in_same_query!(
    events,
    users,
);
