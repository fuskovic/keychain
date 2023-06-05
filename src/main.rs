pub mod args;
use args::{KeychainArgs, KeychainSubcommand};
use clap::Parser;

fn main() {
    let _args = KeychainArgs::parse();
    match &_args.command {
        KeychainSubcommand::Create (input) => {
            println!("{}", input.name)
        }
    }
}