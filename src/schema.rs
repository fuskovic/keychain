// @generated automatically by Diesel CLI.

diesel::table! {
    keychains (id) {
        id -> Nullable<Integer>,
        name -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
