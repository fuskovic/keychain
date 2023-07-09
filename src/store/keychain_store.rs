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
        let result = diesel::insert_into(keychains)
            .values(&kc)
            .execute(&mut self.conn);
        
        match result {
            Ok(_) => { 
                println!("successfully created {}", kc.name) 
            },
            Err(err) => {
                println!("failed to create {}: {}", kc.name, err)
            }
        };
    }
    
    pub fn update(mut self, kc: UpdateKeychain) {
        let result = diesel::update(keychains.find(kc.id))
            .set(&kc)
            .execute(&mut self.conn);

        match result {
            Ok(rows_affected) => {
                if rows_affected == 0 {
                    println!("{} not found", kc.name)
                } else {
                    println!("successfully updated {}", kc.name)
                }
            },
            Err(err) => {
                println!("failed to update {}: {}", kc.name, err)
            }
        };
    }
    
    pub fn list(mut self) {
        let chains = keychains.load::<Keychain>(&mut self.conn).unwrap();
        let _ = print_stdout(chains.with_title());
    }
    
    pub fn delete(mut self, _name: String) {
        let result = diesel::delete(keychains.filter(name.eq(&_name)))
            .execute(&mut self.conn);

        match result {
            Ok(rows_affected) => {
                if rows_affected == 0 {
                    println!("{} not found", _name)
                } else {
                    println!("successfully deleted {}", _name)
                }
            },
            Err(err) => {
                println!("failed to delete {}: {}", _name, err)
            }
        };
    }
}