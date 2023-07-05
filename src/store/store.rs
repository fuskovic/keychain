use diesel::{prelude::*, sqlite::Sqlite};
use std::{path::Path};
use home::{home_dir};
use std::fs;
use std::error::Error;
use super::keychain_store::KeychainStore;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/store/migrations");

pub struct Store {
    pub keychains: KeychainStore,
}

impl Store {
    pub fn new() -> Self {
        let config_dir = Path::new(&home_dir().unwrap())
            .join(".config");

        match fs::create_dir_all(&config_dir){
            Ok(_) => {},
            Err(err) => {
                panic!("Configuration error: {:?}", err)
            },
        };

        let db_url = Path::new(&config_dir).join("krs.db");
        let db_url_str = db_url.to_str().unwrap();
        let mut conn = SqliteConnection::establish(&db_url_str)
            .unwrap_or_else(
                |err| panic!(
                    "failed to connect to {}: {}", &db_url_str, err,
                ),
            );

        run_migrations(&mut conn)
            .unwrap_or_else(|err| panic! (
                "failed to apply migrations: {}", err,
            ),
        );

        Self { keychains: KeychainStore::new(conn) }
    }
}


fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
