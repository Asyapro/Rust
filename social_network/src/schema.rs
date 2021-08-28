table! {
    users (id) {
        id -> Int4,
        info -> Varchar,
        email -> Text,
        password -> Text,
    }
}

table! {
    friends (id) {
        id -> Int4,
        id_user1 -> Int4,
        id_user2 -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    friends,
);
