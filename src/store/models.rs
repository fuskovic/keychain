use diesel::{prelude::*};
use chrono::{NaiveDateTime};
use crate::store::schema::keychains;
use cli_table::{format::Justify, Table};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = keychains)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Table)]
pub struct Keychain {
    #[table(title = "ID", justify = "Justify::Right")]
    pub id: i32,
    #[table(title = "Name", justify = "Justify::Right")]
    pub name: String,
    #[table(title = "Created At", justify = "Justify::Right")]
    pub created_at: NaiveDateTime,
    #[table(title = "Updated At", justify = "Justify::Right")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = keychains)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewKeychain<'a> {
    pub name: &'a String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = keychains)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateKeychain<'a> {
    pub id: &'a i32,
    pub name: &'a String,
    pub updated_at: NaiveDateTime,
}