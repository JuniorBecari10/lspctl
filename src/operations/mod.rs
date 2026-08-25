use crate::{
    error,
    operations::util::{Action, OperationResult, PackageSelection},
    registry::model::Entry,
};

use regex::Regex;

mod logic;
pub mod model;
mod prelude;
pub mod util;

pub fn install(args: model::InstallArgs) -> OperationResult {
    util::run_action(
        PackageSelection::Specific(args.pkgs),
        args.yes,
        true,
        Action::Install,
        logic::install_pkg,
    )
}

pub fn remove(args: model::RemoveArgs) -> OperationResult {
    if !args.all && args.pkgs.is_empty() {
        error!("Specify '-a' / '--all' or one or more package names to remove.");
        return OperationResult::Failure;
    }

    let selection = if args.all {
        PackageSelection::All
    } else {
        PackageSelection::Specific(args.pkgs)
    };

    util::run_action(
        selection,
        args.yes,
        false,
        Action::Remove,
        logic::remove_pkg,
    )
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
    let (registry, _, state, _lock) = prelude::prelude();
    let (entries, missing) = util::filter_registry(registry, &args.pkgs);

    if !missing.is_empty() {
        for m in missing {
            error!("Package '{m}' doesn't exist.");
        }

        return OperationResult::Failure;
    }

    let installed_version = |e: &Entry| state.installed.get(&e.name).map(|pkg| pkg.version.clone());

    for e in entries {
        e.print_detailed(installed_version(&e));
    }

    OperationResult::Success
}
