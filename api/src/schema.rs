table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        name -> Varchar,
        password -> Varchar,
        role -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
