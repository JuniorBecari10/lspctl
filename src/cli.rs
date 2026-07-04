use crate::consts;
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
    Install { name: String },
}

pub fn cli() {
    match Cli::parse().command {
        Command::Install { name } => println!("{name}"),
    }
}
