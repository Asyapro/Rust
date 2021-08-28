table! {
    friends (id) {
        id -> Int4,
        id_user1 -> Int4,
        id_user2 -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        info -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    friends,
    users,
);
