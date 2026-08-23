use clap::Args;

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// List of packages to install
    #[arg(required = true, num_args = 1..)]
    pub pkgs: Vec<String>,

    /// Install the packages even if already installed
    #[arg(short, long)]
    pub force: bool,

    /// Install without confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// List of packages to remove
    #[arg(required = true, num_args = 1..)]
    pub pkgs: Vec<String>,

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
    // The regex pattern to search
    #[arg(required = true)]
    pub pattern: String,

    /// List installed packages instead
    #[arg(short, long)]
    pub installed: bool,

    /// Write more info when listing; this will write more than one line per package
    #[arg(short, long)]
    pub verbose: bool,
}
