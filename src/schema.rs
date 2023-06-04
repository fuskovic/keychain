// @generated automatically by Diesel CLI.

diesel::table! {
    keychains (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
