mod args;
mod db;
mod models;
mod schema;
mod ops;

use args::{KeychainArgs, KeychainSubcommand};
use clap::Parser;
use ops::keychain_ops::update_keychain;
use crate::ops::keychain_ops::{create_keychain, list_keychains};
use models::{NewKeychain, UpdateKeychain};

fn main() {
    let _args = KeychainArgs::parse();
    match &_args.command {
        KeychainSubcommand::Create (input) => {
            create_keychain(NewKeychain {
                name: &input.name 
            })
        }
        KeychainSubcommand::Update (input) => {
            update_keychain(UpdateKeychain {
                // TODO: KeychainSubcommand doesn't support tuple enums so all
                // the subcommands are currently using the same arg input shape
                // with all the same requirements. Figure out how to solve for this.
                id:     &123,
                name:   &input.name
            })
        }
        KeychainSubcommand::List (_) => {
            list_keychains()
        }
    }
}