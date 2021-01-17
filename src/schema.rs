table! {
    events (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
        date -> Date,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    events,
    posts,
);
