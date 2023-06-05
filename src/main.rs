mod args;
mod db;
mod models;
mod schema;
mod ops;

use args::{KeychainArgs, KeychainSubcommand};
use clap::Parser;
use crate::ops::keychain_ops::{create_keychain};
use models::{NewKeychain};

fn main() {
    let _args = KeychainArgs::parse();
    match &_args.command {
        KeychainSubcommand::Create (input) => {
            create_keychain(NewKeychain { 
                name: &input.name 
            })
        }
    }
}