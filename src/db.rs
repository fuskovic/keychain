use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn open() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL unset");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("failed to connect to {}", database_url))
}