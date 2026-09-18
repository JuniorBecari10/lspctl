use clap::{Args, Subcommand, ValueEnum};

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

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFilter {
    Name,
    Version,
    #[value(alias = "desc")]
    Description,
    License,
    #[value(alias = "lang")]
    Language,
    Category,
    Source,
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// The regex pattern to search
    #[arg(required = true)]
    pub pattern: String,

    /// Fields to search; matches if the pattern matches any of the given fields.
    /// Defaults to name only if omitted.
    #[arg(short, long, value_enum, num_args = 1..)]
    pub filters: Vec<SearchFilter>,

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
    SetVersion(RegistrySetVersionArgs),

    /// Sync packages to registry
    #[command(visible_alias = "s")]
    Sync(RegistrySyncArgs),
}

#[derive(Args, Debug)]
pub struct RegistrySetVersionArgs {
    /// Version to set the registry to
    #[arg(short, long)]
    pub version: String,

    #[command(flatten)]
    pub selection: PackageSelectionArgs,

    /// Perform action without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct RegistrySyncArgs {
    #[command(flatten)]
    pub selection: PackageSelectionArgs,

    /// Sync without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

// ---

#[derive(Args, Debug)]
pub struct PackageSelectionArgs {
    /// List of packages to sync with the new registry
    #[arg(conflicts_with = "all")]
    pub pkgs: Vec<String>,

    /// Sync all installed packages
    #[arg(short, long)]
    pub all: bool,
}
