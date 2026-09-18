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

pub fn run_action(
    selection: PackageSelection,
    yes: bool,
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
        Action::Remove => state.installed.keys().cloned().collect(),
    };

    let Ok(entries) = util::filter_registry_print(registry, &pkgs, &installed_pool) else {
        return OperationResult::Failure;
    };

    if !util::accepted_action(&entries, yes, &action, &state) {
        return OperationResult::Success;
    }

    let (mut ok_count, mut err_count, mut skip_count) = (0, 0, 0);

    for pkg in entries {
        if action.should_skip(&state, &pkg.name) {
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
    let (registry, platform, mut state, _lock) = prelude::prelude();

    let pkgs = match selection {
        PackageSelection::Specific(items) => items,
        PackageSelection::All => state.installed.keys().cloned().collect(),
    };

    if pkgs.is_empty() {
        end!("There are no packages to sync.");
        return OperationResult::Success;
    }

    let Ok(entries) = util::filter_registry_print(registry, &pkgs, &pkgs) else {
        return OperationResult::Failure;
    };

    header!("List of packages to sync ({}):", entries.len());

    util::list_entries(&entries, |e| {
        let outdated = state
            .installed
            .get(&e.name)
            .is_some_and(|installed| installed.version == e.source.purl.version);
        outdated.then_some(Marker::Matches)
    });

    if !util::confirm_action("Proceed with sync?", yes) {
        return OperationResult::Success;
    }

    let (mut ok_count, mut err_count, mut skip_count) = (0, 0, 0);

    for entry in entries {
        let name = entry.name.clone();
        let up_to_date = state
            .installed
            .get(&name)
            .is_some_and(|installed| installed.version == entry.source.purl.version);

        if up_to_date {
            step!(
                "Package {} is already up to date. Skipping...",
                name.quote()
            );

            skip_count += 1;
            continue;
        }

        step!("Syncing package {}...", name.quote());
        match logic::install_pkg(entry, &platform, &mut state) {
            Ok(()) => {
                end!("Package synced successfully.");
                ok_count += 1;
            }
            Err(e) => {
                error!("Failed to sync {}: {e}", name.quote());
                err_count += 1;
            }
        }
    }

    let ok_plural = util::plural(ok_count, "package", "packages");
    header!(
        "Successfully synced {ok_count} {ok_plural}. {err_count} had errors. {skip_count} already up to date."
    );

    if err_count == 0 {
        OperationResult::Success
    } else {
        OperationResult::Failure
    }
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
