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

fn execute_entries(
    entries: Vec<Entry>,
    action: &Action,
    op: fn(Entry, &Platform, &mut State) -> anyhow::Result<()>,
    platform: &Platform,
    state: &mut State,
) -> OperationResult {
    let (mut ok_count, mut err_count, mut skip_count) = (0, 0, 0);

    for pkg in entries {
        if action.should_skip(state, &pkg) {
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

        match op(pkg, platform, state) {
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
        Action::Remove | Action::Sync => state.installed.keys().cloned().collect(),
    };

    let Ok(entries) = util::filter_registry_print(registry, &pkgs, &installed_pool) else {
        return OperationResult::Failure;
    };

    if !util::accepted_action(&entries, yes, &action, &state) {
        return OperationResult::Success;
    }

    execute_entries(entries, &action, op, &platform, &mut state)
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

pub fn delete(
    path: &Path,
    already_absent_msg: &str,
    warning: &str,
    fatal_msg: &str,
    yes: bool,
    delete_fn: impl FnOnce(&Path) -> std::io::Result<()>,
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

    delete_fn(path).fatal(fatal_msg);
    OperationResult::Success
}

// TODO: add search for registry versions
pub fn set_registry_version(version: &str, yes: bool) -> OperationResult {
    let (_, _, mut state, _lock) = prelude::prelude();
    let lower_version = version.to_lowercase();

    let release = match registry::fetch_release(&lower_version) {
        Ok(release) => release,
        Err(e) => {
            end_error!("Couldn't get release: {e}");
            return OperationResult::Failure;
        }
    };

    let latest_marker = if lower_version == "latest" {
        " (latest)".cyan().to_string()
    } else {
        String::new()
    };

    step!(
        "Version to be set: {}{}",
        release.tag.quote(),
        latest_marker
    );

    if release.tag == state.registry_tag {
        note!(
            "{} The already installed registry is the same as the one you are going to install.",
            "[!]".yellow()
        );
    }

    if !util::confirm_action("Confirm action?", yes) {
        return OperationResult::Success;
    }

    match registry::install_registry_from_release(&release) {
        Ok(()) => {
            state.set_registry_tag(release.tag);

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

pub fn sync_packages(
    version: Option<String>,
    selection: PackageSelection,
    yes: bool,
) -> OperationResult {
    let (registry, platform, mut state, _lock) = prelude::prelude();

    let pending_release = match version {
        Some(ref v) => {
            let lower_version = v.to_lowercase();

            let release = match registry::fetch_release(&lower_version) {
                Ok(release) => release,
                Err(e) => {
                    end_error!("Couldn't get release: {e}");
                    return OperationResult::Failure;
                }
            };

            let bytes = match registry::fetch_registry_bytes_from_release(&release) {
                Ok(bytes) => bytes,
                Err(e) => {
                    end_error!("Couldn't fetch registry contents: {e}");
                    return OperationResult::Failure;
                }
            };

            Some((release, bytes, lower_version == "latest"))
        }

        None => None,
    };

    if let Some((release, _, is_latest)) = &pending_release {
        let latest_marker = if *is_latest {
            " (latest)".cyan().to_string()
        } else {
            String::new()
        };

        step!(
            "Version to be set: {}{}",
            release.tag.quote(),
            latest_marker
        );

        if release.tag == state.registry_tag {
            note!(
                "{} The already installed registry is the same as the one you are going to install.",
                "[!]".yellow()
            );
        }
    }

    let sync_registry = match &pending_release {
        Some((_, bytes, _)) => match registry::parse_registry_from_bytes(bytes) {
            Ok(r) => r,

            Err(e) => {
                end_error!("Couldn't parse fetched registry: {e}");
                return OperationResult::Failure;
            }
        },

        None => registry, // just get the registry's read one
    };

    let pkgs = match selection {
        PackageSelection::Specific(items) => items,
        PackageSelection::All => state.installed.keys().cloned().collect(),
    };

    if pkgs.is_empty() {
        end!("There are no packages to be synced.");
        return OperationResult::Success;
    }

    let installed_pool: Vec<_> = state.installed.keys().cloned().collect();
    let Ok(entries) = util::filter_registry_print(sync_registry, &pkgs, &installed_pool) else {
        return OperationResult::Failure;
    };

    if !util::accepted_sync(&entries, &state, yes) {
        return OperationResult::Success;
    }

    if let Some((release, bytes, _)) = pending_release {
        if let Err(e) = registry::write_registry_bytes(&bytes) {
            end_error!("Couldn't save new registry: {e}");
            return OperationResult::Failure;
        }

        state.set_registry_tag(release.tag);

        // TODO: revert the old registry?
        if let Err(e) = state.save() {
            end_error!("Couldn't save updated state: {e}");
            return OperationResult::Failure;
        }

        end!("Registry version set successfully.");
    }

    execute_entries(
        entries,
        &Action::Sync,
        logic::install_pkg,
        &platform,
        &mut state,
    )
}

pub fn registry_current() -> OperationResult {
    let (_, _, state, _lock) = prelude::prelude();

    // this time, we can continue even with errors.
    let is_latest = registry::fetch_latest_release()
        .map(|r| r.tag == state.registry_tag)
        .unwrap_or(false);

    let latest_marker = if is_latest {
        " (latest)".cyan().to_string()
    } else {
        String::new()
    };

    end!(
        "Current registry version is {}.{}",
        state.registry_tag.quote(),
        latest_marker
    );

    OperationResult::Success
}

pub fn registry_list(page: u32) -> OperationResult {
    let (_, _, state, _lock) = prelude::prelude();

    let release_tags: Vec<_> = match registry::fetch_release_page(page) {
        Ok(list) => list.into_iter().map(|r| r.tag).collect(),
        Err(e) => {
            end_error!("Couldn't fetch release list: {e}");
            return OperationResult::Failure;
        }
    };

    header!("Listing available registry tags at page {page}:\n");

    util::list_release_tags(&release_tags, &state.registry_tag, page);
    OperationResult::Success
}
