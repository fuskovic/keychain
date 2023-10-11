mod args;
mod store;
use args::{Cli, Commands, KeyCommands, KeysArgs};
use clap::Parser;
use store::{
    store::Store,
    models::{NewKeychain, UpdateKeychain, NewKey},
};
use chrono::offset::Local;
extern crate diesel_migrations;
extern crate cli_table;

fn main() {
    let store = Store::new();
    let _args = Cli::parse();
    match &_args.command {
        Commands::Create { name } => {
            store.keychains.create(NewKeychain { name });
        }
        Commands::Update {id, name} => {
            store.keychains.update(UpdateKeychain { id, name, 
                updated_at: Local::now().naive_local() ,
            })
        }
        Commands::List {} => {
            store.keychains.list()
        }
        Commands::Delete { name } => {
            store.keychains.delete(name.to_string())
        }
        Commands::Keys( keys ) => {
            let keys_cmd = keys.command.as_ref().unwrap();
            match &keys_cmd {
                KeyCommands::Create{
                    keychain_id,
                    key_name,
                    key_value,
                } => {
                    store.keys.create(NewKey{
                        keychain_id: &keychain_id,
                        name: &key_name.to_string(),
                        value: &key_value.to_string()
                    });
                }
                KeyCommands::List { keychain_id } => {
                    store.keys.list(*keychain_id)
                }
            }
        }
    }
}