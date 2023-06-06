use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct KeychainArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    // Create a new keychain
    Create {
        name: String,
    },
    // List existing keychains
    List{},
    // Update/rename existing keychain
    Update {
        id:     i32,
        name:   String,
    },
    // Delete an existing keychain
    Delete {
        name: String,
    }
}

#[derive(Debug, Args)]
pub struct Keychain {
    /// name to assign keychain
    pub name: String,
}