use clap::{Args, Subcommand};

// TODO: add '--all' here?
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

// TODO: add flags to only show some properties, like bins, versions..
#[derive(Args, Debug)]
pub struct ListArgs {
    /// List installed packages instead
    #[arg(short, long)]
    pub installed: bool,

    /// Write more info when listing; this will write more than one line per package
    #[arg(short, long)]
    pub verbose: bool,
}

// TODO: search by description, license, bins.. or any combination of them
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
    /// Set the registry version with optional package syncing
    #[command(visible_alias = "sv")]
    SetVersion(SetVersionRegistryArgs),

    /// Sync packages to registry
    #[command(visible_alias = "s")]
    Sync(SyncRegistryArgs),
}

#[derive(Args, Debug)]
pub struct SetVersionRegistryArgs {
    /// Version to set the registry to
    #[arg(short, long)]
    pub version: String,

    #[command(flatten)]
    pub selection: PackageSelection,

    /// Perform action without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct SyncRegistryArgs {
    #[command(flatten)]
    pub selection: PackageSelection,

    /// Sync without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

// ---

#[derive(Args, Debug)]
pub struct PackageSelection {
    /// List of packages to sync with the new registry
    #[arg(conflicts_with = "all")]
    pub pkgs: Vec<String>,

    /// Sync all installed packages
    #[arg(short, long)]
    pub all: bool,
}
