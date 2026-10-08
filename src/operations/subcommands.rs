use std::{collections::HashMap, fs, path::Path};

use crate::{
    end, end_error, header,
    log::{Fatal, Format},
    note,
    operations::{
        logic,
        model::{Action, OperationResult, PackageSelection, SearchQuery, VersionDisplay},
        prelude,
        util::{self, AcceptSync},
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
    orphaned: &[String],
    action: &Action,
    op: fn(Entry, &Platform, &mut State) -> anyhow::Result<()>,
    platform: &Platform,
    state: &mut State,
) -> OperationResult {
    let (mut ok_count, mut err_count) = (0, 0);

    for pkg in entries {
        if action.should_skip(state, &pkg) {
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

    for name in orphaned {
        step!("{} package {}...", action.gerund(), name.quote());

        match logic::remove(name, state) {
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

    header!(
        "Successfully {} {ok_count} {ok_plural}. {err_count} had errors.",
        action.past_participle(),
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

    let pkgs = util::resolve_selection(&state, selection);

    if pkgs.is_empty() {
        end!("There are no packages to be {}.", action.past_participle());
        return OperationResult::Success;
    }

    let installed_pool = match action {
        Action::Install => registry.0.iter().map(|e| e.name.clone()).collect(),
        Action::Remove | Action::Sync => util::installed_names(&state),
    };

    let (pkgs, orphaned): (Vec<String>, Vec<String>) = if matches!(action, Action::Remove) {
        let registry_names: std::collections::HashSet<&str> =
            registry.0.iter().map(|e| e.name.as_str()).collect();

        pkgs.into_iter()
            .partition(|name| registry_names.contains(name.as_str()))
    } else {
        (pkgs, Vec::new())
    };

    let entries = if pkgs.is_empty() {
        Vec::new()
    } else {
        match util::filter_registry_print(registry, &pkgs, &installed_pool) {
            Ok(entries) => entries,
            Err(_) => return OperationResult::Failure,
        }
    };

    if entries.is_empty() && orphaned.is_empty() {
        end!("There are no packages to be {}.", action.past_participle());
        return OperationResult::Success;
    }

    if !util::accepted_action(&entries, &orphaned, yes, &action, &state) {
        return OperationResult::Success;
    }

    execute_entries(entries, &orphaned, &action, op, &platform, &mut state)
}

pub fn list_packages(
    installed: bool,
    verbose: bool,
    bins: bool,
    versions: bool,
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

        let display = if installed {
            VersionDisplay::Installed
        } else {
            VersionDisplay::RegistryIfMatches
        };

        header!("{header_text}");
        util::write_entries(
            &entries,
            &orphaned,
            &state,
            display,
            verbose,
	    bins,
	    versions,
            &state.installed,
            !installed,
        );
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

    let pending = match util::resolve_release(version) {
        Ok(p) => p,
        Err(e) => {
            end_error!("Couldn't resolve registry version: {e}");
            return OperationResult::Failure;
        }
    };

    util::announce_pending_version(&pending.release, pending.is_latest, &state);

    if !util::confirm_action("Confirm action?", yes) {
        return OperationResult::Success;
    }

    util::commit_registry(&pending.release, &pending.bytes, &mut state)
}

pub fn sync_packages(
    version: Option<String>,
    selection: PackageSelection,
    yes: bool,
) -> OperationResult {
    let (registry, platform, mut state, _lock) = prelude::prelude();

    let pending = match version {
        Some(ref v) => match util::resolve_release(v) {
            Ok(p) => Some(p),
            Err(e) => {
                end_error!("Couldn't resolve registry version: {e}");
                return OperationResult::Failure;
            }
        },
        None => None,
    };

    if let Some(p) = &pending {
        util::announce_pending_version(&p.release, p.is_latest, &state);
    }

    let sync_registry = match &pending {
        Some(p) => match registry::parse_registry_from_bytes(&p.bytes) {
            Ok(r) => r,
            Err(e) => {
                end_error!("Couldn't parse fetched registry: {e}");
                return OperationResult::Failure;
            }
        },
        None => registry,
    };

    let pkgs = util::resolve_selection(&state, selection);

    let entries = if pkgs.is_empty() {
        Vec::new()
    } else {
        let installed_pool = util::installed_names(&state);
        match util::filter_registry_print(sync_registry, &pkgs, &installed_pool) {
            Ok(entries) => entries,
            Err(_) => return OperationResult::Failure,
        }
    };

    match util::accepted_sync(&entries, &state, yes, pending.is_some()) {
        AcceptSync::No => OperationResult::Success,

        yes @ AcceptSync::YesWithPackages | yes @ AcceptSync::YesNoPackages => {
            if let Some(p) = pending
                && let OperationResult::Failure =
                    util::commit_registry(&p.release, &p.bytes, &mut state)
            {
                return OperationResult::Failure;
            }

            if matches!(yes, AcceptSync::YesWithPackages) {
                execute_entries(
                    entries,
                    &[],
                    &Action::Sync,
                    logic::install_pkg,
                    &platform,
                    &mut state,
                )
            } else {
                OperationResult::Success
            }
        }
    }
}

pub fn registry_current() -> OperationResult {
    let (_, _, state, _lock) = prelude::prelude();

    let is_latest = registry::fetch_latest_release()
        .map(|r| r.tag == state.registry_tag)
        .unwrap_or(false);

    end!(
        "Current registry version is {}.{}",
        state.registry_tag.quote(),
        util::latest_marker(is_latest)
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
