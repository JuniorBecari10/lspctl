use crate::{
    error,
    operations::util::{Action, OperationResult},
};

use regex::Regex;

mod logic;
pub mod model;
mod prelude;
pub mod util;

pub fn install(args: model::InstallArgs) -> OperationResult {
    util::run_action(args.pkgs, args.yes, Action::Install, logic::install_pkg)
}

pub fn remove(args: model::RemoveArgs) -> OperationResult {
    util::run_action(args.pkgs, args.yes, Action::Remove, logic::remove_pkg)
}

pub fn list(args: model::ListArgs) -> OperationResult {
    util::list_packages(args.installed, args.verbose, None)
}

pub fn search(args: model::SearchArgs) -> OperationResult {
    let pattern = match Regex::new(&args.pattern) {
        Ok(re) => re,

        Err(e) => {
            error!("Invalid pattern '{}': {e}", args.pattern);
            return OperationResult::Failure;
        }
    };

    util::list_packages(args.installed, args.verbose, Some(&pattern))
}
