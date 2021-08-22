table! {
    auth (id) {
        id -> Int8,
        login -> Varchar,
        auth_type -> Varchar,
        roles -> Array<Text>,
    }
}
