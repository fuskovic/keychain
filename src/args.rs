use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct KeychainArgs {
    #[clap(subcommand)]
    pub command: KeychainSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum KeychainSubcommand {
    // Create a new keychain
    Create(Keychain),
}

#[derive(Debug, Args)]
pub struct Keychain {
    /// name to assign keychain
    pub name: String,
}