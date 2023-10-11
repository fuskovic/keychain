use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new keychain
    Create {
        name: String,
    },
    /// List existing keychains
    List{},
    /// Update/rename existing keychain
    Update {
        id:     i32,
        name:   String,
    },
    /// Delete an existing keychain
    Delete {
        name: String,
    },
    /// Subcommand for managing keychain keys
    Keys(KeysArgs)
}

#[derive(Debug, Args)]
pub struct Keychain {
    /// name to assign keychain
    pub name: String,
}

#[derive(Debug, Args)]
#[command(arg_required_else_help(true))]
pub struct KeysArgs {
    #[command(subcommand)]
    pub command: Option<KeyCommands>,
}

#[derive(Debug, Subcommand)]
pub enum KeyCommands {
    /// Add a new key to an existing keychain
    Create {
        keychain_id: i32,
        key_name: String,
        key_value: String,
    },
    /// List all keys for a given keychain
    List{
        keychain_id: i32,
    },
    /// Update/rename existing keychain
    Update {
        id:     i32,
        key_value: String,
    },
    // /// Delete an existing key
    // Delete {
    //     id: i32,
    // }
}