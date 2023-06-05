use diesel::{prelude::*};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::keychains)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Keychain {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::keychains)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewKeychain<'a> {
    pub name: &'a String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::schema::keychains)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateKeychain<'a> {
    pub id: &'a i32,
    pub name: &'a String,
}