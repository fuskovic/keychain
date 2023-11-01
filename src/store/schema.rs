// @generated automatically by Diesel CLI.

diesel::table! {
    keychains (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    keys (id) {
        id -> Integer,
        keychain_id -> Integer,
        name -> Text,
        value -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}