use std::fs;

use regex::RegexBuilder;

use crate::{
    consts, error,
    log::Format,
    operations::{
        markers::Selection,
        model::{
            Action, DeleteFlags, OperationResult, PackageSelection, RegistryListArgs,
            RegistrySetVersionArgs, RegistrySyncArgs, SearchQuery,
        },
    },
    paths,
    registry::model::Entry,
};

mod logic;
mod markers;
pub mod model;
mod prelude;
mod subcommands;
pub mod util;

pub fn install(args: model::InstallArgs) -> OperationResult {
    subcommands::run_action(
        PackageSelection::Specific(args.pkgs),
        args.yes,
        Action::Install,
        logic::install_pkg,
    )
}

pub fn remove(args: model::RemoveArgs) -> OperationResult {
    let Some(selection) = args.to_package_selection() else {
        return util::selection_error(Action::Remove);
    };

    subcommands::run_action(selection, args.yes, Action::Remove, logic::remove_pkg)
}

pub fn list(args: model::ListArgs) -> OperationResult {
    subcommands::list_packages(args.installed, args.verbose, None)
}

pub fn search(args: model::SearchArgs) -> OperationResult {
    let pattern = match RegexBuilder::new(&args.pattern)
        .case_insensitive(true)
        .build()
    {
        Ok(re) => re,

        Err(e) => {
            error!("Invalid pattern {}: {e}", args.pattern.quote());
            return OperationResult::Failure;
        }
    };

    subcommands::list_packages(
        args.installed,
        args.verbose,
        Some(SearchQuery {
            pattern,
            filters: args.filters,
        }),
    )
}

pub fn info(args: model::InfoArgs) -> OperationResult {
    let (registry, _, state, _lock) = prelude::prelude();

    let pool: Vec<_> = registry.0.iter().map(|e| e.name.clone()).collect();
    let Ok(entries) = util::filter_registry_print(registry, &args.pkgs, &pool) else {
        return OperationResult::Failure;
    };

    let installed_version = |e: &Entry| state.installed.get(&e.name).map(|pkg| pkg.version.clone());

    for e in entries {
        e.print_detailed(installed_version(&e));
    }

    OperationResult::Success
}

pub fn delete_lockfile(flags: DeleteFlags) -> OperationResult {
    subcommands::delete_action(
        &paths::lock_file(),
        "Lockfile is already not present.",
        "This should only be used when the program is in a deadlock and no other instances are running.",
        "Could not delete lockfile.",
        flags.yes,
        |p| fs::remove_file(p),
    )
}

pub fn delete_all(flags: DeleteFlags) -> OperationResult {
    subcommands::delete_action(
        &paths::root_dir(),
        "All data is already not present.",
        &format!("This will delete all data related to {}.", consts::APP_NAME),
        "Could not delete root directory.",
        flags.yes,
        |p| fs::remove_dir_all(p),
    )
}

pub fn registry_set_version(args: RegistrySetVersionArgs) -> OperationResult {
    subcommands::set_registry_version(&args.version, args.yes)
}

pub fn registry_sync(args: RegistrySyncArgs) -> OperationResult {
    let Some(selection) = args.selection.to_package_selection() else {
        return util::selection_error(Action::Sync);
    };

    subcommands::sync_packages(args.version, selection, args.yes)
}

pub fn registry_current() -> OperationResult {
    subcommands::registry_current()
}

pub fn registry_list(args: RegistryListArgs) -> OperationResult {
    subcommands::registry_list(args.page)
}
