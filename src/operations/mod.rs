use std::fs;

use crate::{
    consts, error,
    operations::{
        markers::Selection,
        model::{DeleteFlags, UpdateRegistryArgs},
        util::{Action, OperationResult, PackageSelection},
    },
    paths,
    registry::model::Entry,
};

use regex::Regex;

mod logic;
mod markers;
pub mod model;
mod prelude;
pub mod util;

pub fn install(args: model::InstallArgs) -> OperationResult {
    util::run_action(
        PackageSelection::Specific(args.pkgs),
        args.yes,
        Action::Install,
        logic::install_pkg,
    )
}

pub fn remove(args: model::RemoveArgs) -> OperationResult {
    let Some(selection) = args.to_package_selection() else {
        error!("Specify '-a' / '--all' or one or more package names to remove.");
        return OperationResult::Failure;
    };

    util::run_action(selection, args.yes, Action::Remove, logic::remove_pkg)
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
    util::delete_action(
        &paths::lock_file(),
        "Lockfile is already not present.",
        "This should only be used when the program is in a deadlock and no other instances are running.",
        "Could not delete lockfile.",
        flags.yes,
        |p| fs::remove_file(p),
    )
}

pub fn delete_all(flags: DeleteFlags) -> OperationResult {
    util::delete_action(
        &paths::root_dir(),
        "All data is already not present.",
        &format!("This will delete all data related to {}.", consts::APP_NAME),
        "Could not delete root directory.",
        flags.yes,
        |p| fs::remove_dir_all(p),
    )
}

pub fn update_registry(args: UpdateRegistryArgs) -> OperationResult {
    let (registry, _, state, _lock) = prelude::prelude();
    let (_, missing) = util::filter_registry(registry, &args.pkgs);

    if !missing.is_empty() {
        for m in missing {
            error!("Package '{m}' doesn't exist.");
        }

        return OperationResult::Failure;
    }

    let selection = args.to_package_selection(); // not mandatory to specify packages
    todo!()
}
