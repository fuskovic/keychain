mod args;
mod store;
use args::{KeychainArgs, Command};
use clap::Parser;
use store::{
    store::Store,
    models::{NewKeychain, UpdateKeychain},
};
use chrono::offset::Local;


fn main() {
    let store = Store::new();
    let _args = KeychainArgs::parse();
    match &_args.command {
        Command::Create { name } => {
            store.keychains.create(NewKeychain { name });
        }
        Command::Update {id, name} => {
            store.keychains.update(UpdateKeychain { id, name, 
                updated_at: Local::now().naive_local() ,
            })
        }
        Command::List {} => {
            store.keychains.list()
        }
        Command::Delete { name } => {
            store.keychains.delete(name.to_string())
        }
    }
}