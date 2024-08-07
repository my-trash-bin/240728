// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Text,
        title -> Text,
        content -> Text,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}
