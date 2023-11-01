# Keychain

CLI password manager.

Not for actual use.

Currently just a WIP side project for test-driving [clap](https://github.com/clap-rs/clap) and [diesel](https://github.com/diesel-rs/diesel).

# Install

## Compile from source

    git clone https://github.com/fuskovic/keychain.git
    cd keychain
    cargo install --path .

## Verify install

    keychain -V   

# Usage      

```
Usage: keychain <COMMAND>

Commands:
  create  Create a new keychain
  list    List existing keychains
  update  Update/rename existing keychain
  delete  Delete an existing keychain
  keys    Subcommand for managing keychain keys
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
