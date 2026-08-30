use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// List of packages to install
    #[arg(required = true, num_args = 1..)]
    pub pkgs: Vec<String>,

    /// Install without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// List of packages to remove
    #[arg(conflicts_with = "all")]
    pub pkgs: Vec<String>,

    /// Remove all installed packages
    #[arg(short, long)]
    pub all: bool,

    /// Remove without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// List installed packages instead
    #[arg(short, long)]
    pub installed: bool,

    /// Write more info when listing; this will write more than one line per package
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// The regex pattern to search
    #[arg(required = true)]
    pub pattern: String,

    /// List installed packages instead
    #[arg(short, long)]
    pub installed: bool,

    /// Write more info when listing; this will write more than one line per package
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug)]
pub struct InfoArgs {
    /// List of packages to list information about
    #[arg(required = true)]
    pub pkgs: Vec<String>,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    #[command(subcommand)]
    pub command: DeleteSubcommand,

    #[command(flatten)]
    pub flags: DeleteFlags,
}

#[derive(Subcommand, Debug)]
pub enum DeleteSubcommand {
    /// Delete the lockfile in case of a deadlock
    Lockfile,

    /// Delete every data related to lspctl
    All,
}

#[derive(Args, Debug)]
pub struct DeleteFlags {
    /// Delete without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Subcommand, Debug)]
pub enum RegistrySubcommand {
    /// Update the registry with optional package syncing
    #[command(visible_alias = "u")]
    Update(UpdateRegistryArgs),
}

#[derive(Args, Debug)]
pub struct UpdateRegistryArgs {
    /// List of packages to sync with the new registry
    #[arg(conflicts_with = "all")]
    pub pkgs: Vec<String>,

    /// Sync all installed packages
    #[arg(short, long)]
    pub all: bool,

    /// Sync without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,

    /// Version specification of the new registry
    #[arg(short, long)]
    pub version: String,
}
