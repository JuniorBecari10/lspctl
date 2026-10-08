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

#[derive(Args, Debug)]
pub struct ListArgs {
    /// List installed packages instead
    #[arg(short, long)]
    pub installed: bool,

    /// Write more info when listing; this will write more than one line per package
    #[arg(short, long)]
    pub verbose: bool,

    /// Show only package binaries
    #[arg(long)]
    pub bins: bool,

    /// Show only package versions
    #[arg(long)]
    pub versions: bool,
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
    /// Defaults to 'name' if omitted.
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

    /// Print currently installed registry
    #[command(visible_alias = "c")]
    Current,

    /// List available registry versions
    #[command(visible_alias = "l")]
    List(RegistryListArgs),
}

#[derive(Args, Debug)]
pub struct RegistrySetVersionArgs {
    /// Version to set the registry to (or 'latest')
    pub version: String,

    /// Perform action without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct RegistrySyncArgs {
    /// Version to set the registry to (or 'latest'). Leave blank to sync to current version
    #[arg(short, long)]
    pub version: Option<String>,

    #[command(flatten)]
    pub selection: PackageSelectionArgs,

    /// Sync without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct RegistryListArgs {
    /// Page number of the list
    #[arg(short, long, default_value_t = 1)]
    pub page: u32,
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
