use std::fs;

use crate::{
    end, error,
    log::Fatal,
    note,
    operations::{
        model::DeleteFlags,
        util::{Action, OperationResult, PackageSelection},
    },
    paths,
    registry::model::Entry,
    step,
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

pub fn delete_lockfile(flags: DeleteFlags) -> OperationResult {
    if !flags.yes {
        step!("Proceed with lockfile deletion?");
        note!(
            "This should only be used when the program is in a deadlock and no other instances are running."
        );
    }

    if !util::confirm_action("Proceed?", flags.yes) {
        return OperationResult::Success;
    }

    // if an error occurs here, it should be caught by the remove_file call.
    if let Ok(false) = fs::exists(paths::lock_file()) {
        end!("Lockfile is already not present.");
        return OperationResult::Success;
    }

    fs::remove_file(paths::lock_file()).fatal("Could not delete lockfile.");
    OperationResult::Success
}
