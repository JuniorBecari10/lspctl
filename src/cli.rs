use crate::{consts, operations};
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
    /// Install packages
    #[command(visible_alias = "i")]
    Install(operations::model::InstallArgs),

    /// Remove packages
    #[command(visible_alias = "r")]
    Remove(operations::model::RemoveArgs),

    /// List all packages or installed ones
    #[command(visible_alias = "l")]
    List(operations::model::ListArgs),
}

pub fn cli() {
    match Cli::parse().command {
        Command::Install(args) => operations::install(args),
        Command::Remove(args) => operations::remove(args),
        Command::List(args) => operations::list(args),
    }
}
