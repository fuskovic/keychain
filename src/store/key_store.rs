use diesel::prelude::*;
use crate::{
    store::models::{NewKey, Key, UpdateKey}, 
    store::schema::keys::{dsl::keys, name, keychain_id},
};
use cli_table::{print_stdout, WithTitle};

pub struct KeyStore {
    conn: SqliteConnection,
}

impl KeyStore {
    pub fn new(conn: SqliteConnection) -> KeyStore {
        Self { conn }
    }

    pub fn create(mut self, k: NewKey) {
        match diesel::insert_into(keys)
            .values(&k)
            .execute(&mut self.conn) {
            Ok(_) => { 
                println!("successfully created {}", k.name) 
            },
            Err(err) => {
                println!("failed to create {}: {}", k.name, err)
            }
        };
    }
    
    pub fn update(mut self, k: UpdateKey) {
        match diesel::update(keys.find(k.id))
            .set(&k)
            .execute(&mut self.conn) {
            Ok(rows_affected) => {
                if rows_affected == 0 {
                    println!("key id {} not found", k.id)
                } else {
                    println!("successfully updated key id {}", k.id)
                }
            },
            Err(err) => {
                println!("failed to update key id {}: {}", k.id, err)
            }
        };
    }
    
    pub fn list(mut self, chain_id: i32) {
        let chain_keys = keys.
            filter(keychain_id.eq(chain_id))
            .load::<Key>(&mut self.conn)
            .unwrap();
        let _ = print_stdout(chain_keys.with_title());
    }
    
    pub fn delete(mut self, _name: String) {
        match diesel::delete(keys.filter(name.eq(&_name)))
            .execute(&mut self.conn) {
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