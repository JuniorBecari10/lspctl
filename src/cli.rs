use crate::{
    consts,
    operations::{
        self,
        model::{DeleteSubcommand, InfoArgs, InstallArgs, ListArgs, RemoveArgs, SearchArgs},
        util::OperationResult,
    },
};
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
    Install(InstallArgs),

    /// Remove packages
    #[command(visible_alias = "r")]
    Remove(RemoveArgs),

    /// List all packages or installed ones
    #[command(visible_alias = "l")]
    List(ListArgs),

    /// Search all packages or installed ones
    #[command(visible_alias = "s")]
    Search(SearchArgs),

    /// Get more information about packages
    Info(InfoArgs),

    /// Deletion-related utilities
    #[command(visible_alias = "d")]
    Delete {
        #[command(subcommand)]
        command: DeleteSubcommand,
    },
}

pub fn cli() -> OperationResult {
    match Cli::parse().command {
        Command::Install(args) => operations::install(args),
        Command::Remove(args) => operations::remove(args),
        Command::List(args) => operations::list(args),
        Command::Search(args) => operations::search(args),
        Command::Info(args) => operations::info(args),

        Command::Delete { command } => match command {
            DeleteSubcommand::Registry => todo!(),
            DeleteSubcommand::Packages => todo!(),
            DeleteSubcommand::Lockfile => todo!(),
        },
    }
}
