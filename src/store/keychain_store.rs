use diesel::prelude::*;
use crate::{
    store::models::{NewKeychain, Keychain, UpdateKeychain}, 
    store::schema::keychains::{dsl::keychains, name},
};
use cli_table::{print_stdout, WithTitle};

pub struct KeychainStore {
    conn: SqliteConnection,
}

impl KeychainStore {
    pub fn new(conn: SqliteConnection) -> KeychainStore {
        Self { conn }
    }

    pub fn create(mut self, kc: NewKeychain) {
        diesel::insert_into(keychains)
            .values(&kc)
            .execute(&mut self.conn)
            .expect("failed to insert keychain");
    }
    
    pub fn update(mut self, kc: UpdateKeychain) {
        diesel::update(keychains.find(kc.id))
            .set(&kc)
            .execute(&mut self.conn)
            .expect("failed to update keychain");
    }
    
    pub fn list(mut self) {
        let chains = keychains.load::<Keychain>(&mut self.conn).unwrap();
        chains.with_title();
        let _ = print_stdout(chains.with_title());
    }
    
    pub fn delete(mut self, _name: String) {
        diesel::delete(keychains.filter(name.eq(_name)))
            .execute(&mut self.conn)
            .expect("failed to delete keychain");
    }
}