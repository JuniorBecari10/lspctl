use crate::{
    error,
    operations::util::{Action, OperationResult},
    registry::model::Entry,
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

pub fn info(args: model::InfoArgs) -> OperationResult {
    let (registry, _, state, _lock) = prelude::prelude_no_log();
    let (entries, missing) = util::filter_registry(registry, &args.pkgs);

    if !missing.is_empty() {
        for m in missing {
            error!("Package '{m}' doesn't exist.");
        }

        return OperationResult::Failure;
    }
    let is_installed = |e: &Entry| state.installed.contains_key(&e.name);

    for e in entries {
        e.print_detailed(is_installed(&e));
    }

    OperationResult::Success
}
