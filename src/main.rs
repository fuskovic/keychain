mod args;
mod db;
mod models;
mod schema;
mod ops;

use args::{KeychainArgs, Command};
use clap::Parser;
use crate::ops::keychain_ops::{
    create_keychain, 
    update_keychain, 
    list_keychains,
    delete_keychain
};
use models::{NewKeychain, UpdateKeychain};

fn main() {
    let _args = KeychainArgs::parse();
    match &_args.command {
        Command::Create { name } => {
            create_keychain(NewKeychain { name })
        }
        Command::Update {id, name} => {
            update_keychain(UpdateKeychain { id, name, 
                updated_at: Some(chrono::offset::Local::now().naive_local()) ,
            })
        }
        Command::List {} => {
            list_keychains()
        }
        Command::Delete { name } => {
            delete_keychain(name.to_string())
        }
    }
}