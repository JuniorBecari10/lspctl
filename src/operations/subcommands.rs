use std::{fs, path::Path};

use crate::{
    end, end_error, error, header,
    log::{Fatal, Format},
    note,
    operations::{
        logic,
        model::{Action, Marker, OperationResult, PackageSelection, SearchQuery},
        prelude, util,
    },
    registry::model::{Entry, Platform},
    state::State,
    step,
};

// TODO: make the force flag visual in installer
pub fn run_action(
    selection: PackageSelection,
    yes: bool,
    force: bool,
    action: Action,
    op: fn(Entry, &Platform, &mut State) -> anyhow::Result<()>,
) -> OperationResult {
    let (registry, platform, mut state, _lock) = prelude::prelude();

    let pkgs = match selection {
        PackageSelection::Specific(items) => items,
        PackageSelection::All => state.installed.keys().cloned().collect(),
    };

    if pkgs.is_empty() {
        end!("There are no packages to be {}.", action.past_participle());
        return OperationResult::Success;
    }

    let installed_pool: Vec<_> = match action {
        Action::Install => registry.0.iter().map(|e| e.name.clone()).collect(),
        Action::Remove | Action::Sync => state.installed.keys().cloned().collect(),
    };

    let Ok(entries) = util::filter_registry_print(registry, &pkgs, &installed_pool) else {
        return OperationResult::Failure;
    };

    if !util::accepted_action(&entries, yes, &action, &state) {
        return OperationResult::Success;
    }

    let (mut ok_count, mut err_count, mut skip_count) = (0, 0, 0);
    let not_forced = !matches!(action, Action::Install) || !force; // only applies to install

    for pkg in entries {
        if not_forced && action.should_skip(&state, &pkg) {
            step!(
                "Package {} {}. Skipping...",
                pkg.name.quote(),
                action.skip_reason()
            );
            skip_count += 1;
            continue;
        }

        let name = pkg.name.clone();
        step!("{} package {}...", action.gerund(), pkg.name.quote());

        match op(pkg, &platform, &mut state) {
            Ok(()) => {
                end!("Package {} successfully.", action.past_participle());
                ok_count += 1;
            }

            Err(e) => {
                end_error!("Failed to {} {}: {e}", action.verb_base(), name.quote());
                err_count += 1;
            }
        }
    }

    let ok_plural = util::plural(ok_count, "package", "packages");
    let skip_plural = util::plural(skip_count, "was", "were");

    header!(
        "Successfully {} {ok_count} {ok_plural}. {err_count} had errors. {skip_count} {skip_plural} {}.",
        action.past_participle(),
        action.skip_tally_word(),
    );

    if err_count == 0 {
        OperationResult::Success
    } else {
        OperationResult::Failure
    }
}

// TODO: get data from installed packages only when listing or fetching data locally
pub fn list_packages(
    installed: bool,
    verbose: bool,
    query: Option<SearchQuery>,
) -> OperationResult {
    let (registry, _, state, _lock) = prelude::prelude();

    let entries: Vec<Entry> = if installed {
        let keys = state.installed.keys().cloned().collect::<Vec<_>>();
        let pool: Vec<_> = registry.0.iter().map(|e| e.name.clone()).collect();

        let Ok(found) = util::filter_registry_print(registry, keys.as_slice(), &pool) else {
            return OperationResult::Failure;
        };

        found
    } else {
        registry.0
    };

    let entries: Vec<Entry> = match query {
        Some(ref q) => entries.into_iter().filter(|e| q.matches(e)).collect(),
        None => entries,
    };

    if entries.is_empty() {
        let field_suffix = query
            .as_ref()
            .and_then(|q| {
                if q.filters.is_empty() {
                    None
                } else {
                    Some(
                        q.filters
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                }
            })
            .map(|f| format!(" in {f}"))
            .unwrap_or_default();

        let msg = match (installed, query) {
            (true, Some(q)) => format!(
                "No installed packages match '{}'{field_suffix}.",
                q.pattern.as_str()
            ),
            (true, None) => "There are no packages installed.".to_string(),
            (false, Some(q)) => {
                format!(
                    "No packages match {}{field_suffix}.",
                    q.pattern.as_str().quote()
                )
            }
            (false, None) => "No packages found.".to_string(),
        };

        end!("{msg}");
        return OperationResult::Success;
    }

    let header_text = match (installed, query.is_some()) {
        (true, true) => "All matching installed packages:\n",
        (true, false) => "Installed packages:\n",
        (false, true) => "All matching packages:\n",
        (false, false) => "All packages:\n",
    };

    header!("{header_text}");

    util::write_entries(&entries, verbose, &state.installed, !installed);
    OperationResult::Success
}

pub fn sync_packages(selection: PackageSelection, yes: bool) -> OperationResult {
    run_action(selection, yes, true, Action::Sync, logic::install_pkg)
}

pub fn delete_action(
    path: &Path,
    already_absent_msg: &str,
    warning: &str,
    fatal_msg: &str,
    yes: bool,
    delete: impl FnOnce(&Path) -> std::io::Result<()>,
) -> OperationResult {
    if let Ok(false) = fs::exists(path) {
        end!("{already_absent_msg}");
        return OperationResult::Success;
    }

    if !yes {
        step!("Proceed with deletion?");
        note!("{warning}");
    }

    if !util::confirm_action("Proceed?", yes) {
        return OperationResult::Success;
    }

    delete(path).fatal(fatal_msg);
    OperationResult::Success
}
