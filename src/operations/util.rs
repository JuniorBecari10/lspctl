use std::collections::HashSet;

use colored::Colorize;
use dialoguer::Confirm;

use crate::{
    end, end_error, header,
    log::Format,
    note,
    operations::model::{Action, Marker, OperationResult, PackageSelection, VersionDisplay},
    registry::{
        self,
        model::{Entry, Registry, Release},
    },
    state::State,
    step,
    table::{self, Table, cell::Cell, row::Row},
};

const SUGGESTION_THRESHOLD: f64 = 0.7;
const MAX_SUGGESTIONS: usize = 3;

pub enum AcceptSync {
    No,
    YesWithPackages,
    YesNoPackages,
}

pub struct PendingRelease {
    pub release: Release,
    pub bytes: Vec<u8>,
    pub is_latest: bool,
}

fn suggest_similar<'a>(name: &str, pool: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
    let mut scored: Vec<(f64, &str)> = pool
        .map(|candidate| (strsim::jaro_winkler(name, candidate), candidate))
        .filter(|(score, _)| *score >= SUGGESTION_THRESHOLD)
        .collect();

    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    scored
        .into_iter()
        .take(MAX_SUGGESTIONS)
        .map(|(_, n)| n)
        .collect()
}

pub fn accepted_action(
    pkgs: &[Entry],
    orphaned: &[String],
    yes: bool,
    action: &Action,
    state: &State,
) -> bool {
    let will_act = |e: &Entry| -> bool {
        let installed = state.get_package(&e.name);

        match action {
            Action::Remove => installed.is_some(),
            Action::Sync => installed.is_none_or(|p| p.version != e.source.purl.version),
            Action::Install => installed.is_none_or(|p| p.version != e.source.purl.version),
        }
    };

    let action_count = pkgs.iter().filter(|e| will_act(e)).count();

    header!(
        "Packages to be {} ({}):\n",
        action.past_participle(),
        action_count
    );

    list_entries(pkgs, orphaned, state, action.to_display(), |e| {
        let installed = state.get_package(&e.name);

        match action {
            Action::Remove => installed.is_none().then(|| action.marker()),

            Action::Sync => installed
                .is_some_and(|p| p.version == e.source.purl.version)
                .then(|| action.marker()),

            Action::Install => match installed {
                Some(p) if p.version == e.source.purl.version => Some(action.marker()),
                Some(_) => Some(Marker::NotSynced),
                None => None,
            },
        }
    });

    if action_count == 0 {
        eprintln!();

        end!(
            "All {} package(s) are already {}.",
            pkgs.len(),
            action.past_participle()
        );
        return false;
    }

    confirm_action(&format!("Proceed with {}?", action.noun()), yes)
}

pub fn confirm_action(action: &str, yes: bool) -> bool {
    yes || {
        eprintln!();
        Confirm::new()
            .with_prompt(format!(" {} {}", "-".green(), action))
            .default(true)
            .interact()
            .unwrap_or(false)
    }
}

pub fn filter_registry(registry: Registry, pkgs: &[String]) -> (Vec<Entry>, Vec<&str>) {
    let wanted: HashSet<&str> = pkgs.iter().map(String::as_str).collect();

    let found: Vec<Entry> = registry
        .0
        .into_iter()
        .filter(|e| wanted.contains(e.name.as_str()))
        .collect();

    let found_names: HashSet<&str> = found.iter().map(|e| e.name.as_str()).collect();

    let missing: Vec<&str> = pkgs
        .iter()
        .map(String::as_str)
        .filter(|name| !found_names.contains(name))
        .collect();

    (found, missing)
}

pub fn filter_registry_print(
    registry: Registry,
    pkgs: &[String],
    suggest_pool: &[String],
) -> Result<Vec<Entry>, ()> {
    let (entries, missing) = filter_registry(registry, pkgs);

    if missing.is_empty() {
        return Ok(entries);
    }

    for m in missing {
        let suggestions = suggest_similar(
            m,
            suggest_pool.iter().map(String::as_str).filter(|p| *p != m),
        );

        if suggestions.is_empty() {
            end_error!(
                "Package {} doesn't exist in the current registry.",
                m.quote()
            );
        } else {
            let list = suggestions
                .iter()
                .map(|s| s.quote().to_string())
                .collect::<Vec<_>>()
                .join(", ");

            end_error!(
                "Package {} doesn't exist in the current registry. Did you mean: {list}?",
                m.quote()
            );
        }
    }

    Err(())
}

pub const fn plural<'a>(count: i32, singular: &'a str, plural: &'a str) -> &'a str {
    if count == 1 { singular } else { plural }
}

pub fn write_entries(
    entries: &[Entry],
    orphaned: &[String],
    state: &State,
    display: VersionDisplay,
    verbose: bool,
    show_marker: bool,
) {
    let installed_version = |e: &Entry| state.get_package(&e.name).map(|pkg| pkg.version.clone());

    if verbose {
        for entry in entries {
            entry.print_detailed(installed_version(entry), show_marker);
        }
    } else {
        list_entries(entries, orphaned, state, display, |e| {
            (show_marker && state.package_exists(&e.name)).then_some(Marker::Installed)
        });
    }
}

pub fn list_entries(
    entries: &[Entry],
    orphaned: &[String],
    state: &State,
    display: VersionDisplay,
    marker: impl Fn(&Entry) -> Option<Marker>,
) {
    let installed_version = |e: &Entry| state.get_package(&e.name).map(|p| p.version.clone());

    let show_divergence = matches!(display, VersionDisplay::RegistryIfMatches)
        && entries
            .iter()
            .any(|e| installed_version(e).is_some_and(|v| v != e.source.purl.version));

    let mut table = Table::new().column("Name", table::LONG_COL);
    table = if show_divergence {
        table
            .column("Current", table::LONG_COL)
            .column("Registry", table::LONG_COL)
    } else {
        table.column("Version", table::LONG_COL)
    };

    let mut table = table.column("Source", table::SHORT_COL);

    // Shared by registry entries and orphans. None = no value
    let build_row = |name: Cell,
                     current: Option<String>,
                     registry: Option<String>,
                     source: Option<String>,
                     marker: Option<Marker>| {
        let mut cells = vec![name];

        if show_divergence {
            cells.push(current.map(|v| v.cyan()).into());
            cells.push(registry.map(|v| v.green()).into());
        } else {
            let version = match display {
                VersionDisplay::Installed => current,
                VersionDisplay::RegistryIfMatches => registry,
            };

            cells.push(version.map(|v| v.cyan()).into());
        }

        cells.push(source.map(|s| s.dimmed()).into());
        Row::new(cells).marker(marker)
    };

    for e in entries {
        let name = if e.deprecation.is_some() {
            e.name.strikethrough().dimmed()
        } else {
            e.name.normal()
        };

        table.push_row(build_row(
            name.into(),
            installed_version(e),
            Some(e.source.purl.version.clone()),
            Some(e.source.purl.kind.to_string()),
            marker(e),
        ));
    }

    for name in orphaned {
        // Orphans only have installed info, and only in `Installed` mode.
        let installed = match display {
            VersionDisplay::Installed => state.get_package(name),
            VersionDisplay::RegistryIfMatches => None,
        };

        table.push_row(build_row(
            name.as_str().into(),
            installed.map(|p| p.version.clone()),
            None,
            installed.map(|p| p.install_kind.to_string()),
            Some(Marker::NotInRegistry),
        ));
    }

    table.print();
}

pub fn list_release_tags(tags: &[String], current: &str, page: u32) {
    let mut table = Table::new().column("Tag", table::LONG_COL);

    for (i, tag) in tags.iter().enumerate() {
        let is_latest = page <= 1 && i == 0;
        let is_installed = tag == current;

        let cell: Cell = match (is_latest, is_installed) {
            (_, true) => tag.green().into(),
            (true, false) => tag.cyan().into(),
            (false, false) => tag.as_str().into(),
        };

        let markers = [
            (is_latest, Marker::Latest),
            (is_installed, Marker::Installed),
        ]
        .into_iter()
        .filter_map(|(on, m)| on.then_some(m));

        table.push_row(Row::new([cell]).markers(markers));
    }

    table.print();
}

pub fn selection_error(a: Action) -> OperationResult {
    end_error!(
        "Specify {} / {} or one or more package names to {}.",
        "-a".quote(),
        "--all".quote(),
        a.verb_base(),
    );

    OperationResult::Failure
}

pub fn accepted_sync(
    entries: &[Entry],
    state: &State,
    yes: bool,
    pending_version: bool,
) -> AcceptSync {
    let needs_sync = |e: &Entry| -> bool { !Action::Sync.should_skip(state, e) };
    let action_count = entries.iter().filter(|e| needs_sync(e)).count();

    if action_count == 0 {
        if pending_version {
            eprintln!();

            if entries.is_empty() {
                end!("No packages to sync.");
            } else {
                end!("No packages need to be synced with the new registry version.");
            }

            if confirm_action(&format!("Proceed with {}?", Action::Sync.noun()), yes) {
                return AcceptSync::YesNoPackages;
            }
        }

        if entries.is_empty() {
            end!("There are no packages to be synced.");
        } else {
            end!("All {} package(s) are already synced.", entries.len());
        }

        return AcceptSync::YesNoPackages;
    }

    header!(
        "Packages to be {} ({}):\n",
        Action::Sync.past_participle(),
        action_count
    );

    let rows: Vec<_> = entries
        .iter()
        .map(|e| {
            let current = state
                .get_package(&e.name)
                .map(|installed| installed.version.clone())
                .unwrap_or_else(|| "?".into());

            let synced = !needs_sync(e);

            let registry = if synced {
                "—".to_string()
            } else {
                e.source.purl.version.clone()
            };

            (e.name.clone(), current, registry, synced)
        })
        .collect();

    let width = |s: &str| s.chars().count();

    let name_width = rows
        .iter()
        .map(|(n, _, _, _)| width(n))
        .max()
        .unwrap_or(0)
        .max(15);

    let current_width = rows
        .iter()
        .map(|(_, c, _, _)| width(c))
        .max()
        .unwrap_or(0)
        .max(15);

    let registry_width = rows
        .iter()
        .map(|(_, _, r, _)| width(r))
        .max()
        .unwrap_or(0)
        .max(8);

    let pad = |s: &str, w: usize| format!("{s}{}", " ".repeat(w.saturating_sub(width(s))));

    println!(
        "{}  {}  {}",
        pad("Name", name_width).bold(),
        pad("Current", current_width).bold(),
        pad("Registry", registry_width).bold(),
    );
    println!(
        "{}",
        "─"
            .repeat(name_width + current_width + registry_width + 4)
            .dimmed()
    );

    for (name, current, registry, synced) in &rows {
        let marker = if *synced {
            format!("  {}", Marker::Synced.render())
        } else {
            String::new()
        };

        let registry_cell = if *synced {
            pad(registry, registry_width).dimmed().to_string()
        } else {
            pad(registry, registry_width).green().to_string()
        };

        println!(
            "{}  {}  {}{}",
            pad(name, name_width),
            pad(current, current_width).cyan(),
            registry_cell,
            marker,
        );
    }

    if confirm_action(&format!("Proceed with {}?", Action::Sync.noun()), yes) {
        AcceptSync::YesWithPackages
    } else {
        AcceptSync::No
    }
}

pub fn latest_marker(is_latest: bool) -> String {
    if is_latest {
        format!(" {}", Marker::Latest.render())
    } else {
        String::new()
    }
}

fn installed_marker(is_installed: bool) -> String {
    if is_installed {
        format!(" {}", Marker::Installed.render())
    } else {
        String::new()
    }
}

pub fn installed_names(state: &State) -> Vec<String> {
    state.names().cloned().collect()
}

pub fn resolve_selection(state: &State, selection: PackageSelection) -> Vec<String> {
    match selection {
        PackageSelection::Specific(items) => items,
        PackageSelection::All => installed_names(state),
    }
}

// TODO: what is this
pub fn resolve_release(version: &str) -> anyhow::Result<PendingRelease> {
    let lower = version.to_lowercase();
    let release = registry::fetch_release(&lower)?;
    let bytes = registry::fetch_registry_bytes_from_release(&release)?;

    Ok(PendingRelease {
        release,
        bytes,
        is_latest: lower == "latest",
    })
}

pub fn announce_pending_version(release: &Release, is_latest: bool, state: &State) {
    let is_installed = release.tag == state.registry_tag;

    step!(
        "Version to be set for registry: {}{}{}",
        release.tag.quote(),
        latest_marker(is_latest),
        installed_marker(is_installed)
    );

    if is_installed {
        note!(
            "{} Version already installed; proceeding will re-download the same registry.",
            "[!]".yellow()
        );
    }
}

// TODO: don't perform the action if the registry is the same.
// or do it, maybe optionally and to keep consistency bring --force back.
pub fn commit_registry(release: Release, bytes: &[u8], state: &mut State) -> OperationResult {
    if let Err(e) = registry::write_registry_to_disk(bytes) {
        end_error!("Cannot save new registry: {e}");
        return OperationResult::Failure;
    }

    state.registry_tag = release.tag;

    if let Err(e) = state.save() {
        end_error!("Cannot save updated state: {e}");
        return OperationResult::Failure;
    }

    end!("Registry version set successfully.");
    OperationResult::Success
}
