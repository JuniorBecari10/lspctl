use clap::Args;

#[derive(Args)]
pub struct InstallArgs {
    /// List of packages to install
    pub pkgs: Vec<String>,

    /// Install the packages even if already installed
    #[arg(short, long)]
    pub force: bool,

    /// Install without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// List of packages to remove
    pub pkgs: Vec<String>,

    /// Install the packages even if already installed
    #[arg(short, long)]
    pub force: bool,

    /// Install without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}
