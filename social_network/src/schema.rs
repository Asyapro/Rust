table! {
    users (id) {
        id -> Int4,
        info -> Varchar,
        friends -> Nullable<Array<Int4>>,
        email -> Text,
        password -> Text,
    }
}

table! {
    users_auth (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    users_auth,
);
