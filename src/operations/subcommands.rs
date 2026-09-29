use std::{collections::HashMap, fs, path::Path};

use colored::Colorize;

use crate::{
    end, end_error, header,
    log::{Fatal, Format},
    note,
    operations::{
        logic,
        model::{Action, OperationResult, PackageSelection, SearchQuery},
        prelude, util,
    },
    registry::{
        self,
        model::{Entry, Platform},
    },
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

pub fn list_packages(
    installed: bool,
    verbose: bool,
    query: Option<SearchQuery>,
) -> OperationResult {
    let (registry, _, state, _lock) = prelude::prelude();

    let (entries, orphaned): (Vec<Entry>, Vec<String>) = if installed {
        let mut by_name: HashMap<String, Entry> = registry
            .0
            .into_iter()
            .map(|e| (e.name.clone(), e))
            .collect();

        let mut entries = Vec::new();
        let mut orphaned = Vec::new();

        for name in state.installed.keys() {
            match by_name.remove(name) {
                Some(entry) => entries.push(entry),
                None => orphaned.push(name.clone()),
            }
        }

        (entries, orphaned)
    } else {
        (registry.0, Vec::new())
    };

    let entries: Vec<Entry> = match query {
        Some(ref q) => entries.into_iter().filter(|e| q.matches(e)).collect(),
        None => entries,
    };

    let orphaned: Vec<String> = match query {
        Some(ref q) => orphaned
            .into_iter()
            .filter(|name| q.matches_name(name))
            .collect(),
        None => orphaned,
    };

    if entries.is_empty() && orphaned.is_empty() {
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

    if !entries.is_empty() {
        let header_text = match (installed, query.is_some()) {
            (true, true) => "All matching installed packages:\n",
            (true, false) => "Installed packages:\n",
            (false, true) => "All matching packages:\n",
            (false, false) => "All packages:\n",
        };

        header!("{header_text}");
        util::write_entries(&entries, verbose, &state.installed, !installed);
    }

    if !orphaned.is_empty() {
        header!("Installed but not found in registry:\n");
        for name in &orphaned {
            println!("  {name}");
        }
    }

    OperationResult::Success
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

// if the registry has already the version you are gonna install, show a message about that
// if the spec is 'latest' or matches the latest one available.
// TODO: add search for registry versions
pub fn set_registry_version(version: &str, yes: bool) -> OperationResult {
    let (_, _, mut state, _lock) = prelude::prelude();
    let version = version.to_lowercase();

    let release_tag = match registry::get_release_data(&version) {
        Ok(release) => release.tag_name,
        Err(e) => {
            end_error!("Couldn't get release: {e}");
            return OperationResult::Failure;
        }
    };

    step!("Version to be set: {}", release_tag.quote());

    if release_tag == state.registry_tag {
        note!(
            "{} The already installed registry is the same as the one you are going to install.",
            "[!]".yellow()
        );
    }

    if !util::confirm_action("Confirm action?", yes) {
        return OperationResult::Success;
    }

    // this does the job again of getting a Release
    match registry::get_registry_release(&version) {
        Ok(tag) => {
            state.set_registry_tag(tag);

            // TODO: revert the old registry?
            if let Err(e) = state.save() {
                end_error!("Couldn't save updated state: {e}");
                return OperationResult::Failure;
            }

            end!("Registry version set successfully.");
        }

        Err(e) => {
            end_error!("Couldn't set version of registry: {e}");
            return OperationResult::Failure;
        }
    }

    OperationResult::Success
}

pub fn sync_packages(selection: PackageSelection, yes: bool) -> OperationResult {
    run_action(selection, yes, true, Action::Sync, logic::install_pkg)
}

pub fn registry_current() -> OperationResult {
    let (_, _, state, _lock) = prelude::prelude();

    // this time, we can continue even with errors
    let latest_tag = registry::get_release_data("latest")
        .map(|rel| rel.tag_name)
        .unwrap_or_default();

    step!(
        "Current registry version is {}{}",
        state.registry_tag.quote(),
        if state.registry_tag == latest_tag {
            " (latest)".italic()
        } else {
            "".into()
        }
    );

    OperationResult::Success
}

pub fn registry_list(page: u32) -> OperationResult {
    let (_, _, state, _lock) = prelude::prelude();
    let releases = match registry::get_release_list(page) {
        Ok(list) => list,
        Err(e) => {
            end_error!("Couldn't fetch release list: {e}");
            return OperationResult::Failure;
        }
    };

    // write releases and mark latest as page = 1 and index = 0, and the current one, if present
    OperationResult::Success
}
