use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use super::keychain_store::KeychainStore;


pub struct Store {
    pub keychains: KeychainStore,
}

impl Store {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL unset");
        let conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("failed to connect to {}", database_url));
        let keychain_store = KeychainStore::new(conn);
        Self { keychains: keychain_store }
    }
}
