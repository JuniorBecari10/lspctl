use crate::{consts, operations, root};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = consts::APP_NAME,
    version = consts::APP_VERSION,
    about = consts::APP_DESC,
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Install { pkgs: Vec<String> },
}

pub fn cli() {
    root::setup_root().expect("Cannot create root folder structure");

    match Cli::parse().command {
        Command::Install { pkgs } => operations::install(pkgs),
    }
}
