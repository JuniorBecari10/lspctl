use std::collections::{HashMap, HashSet};

use colored::Colorize;
use dialoguer::Confirm;

use crate::{
    end, end_error, header,
    log::Format,
    note,
    operations::model::{Action, Marker, OperationResult, PackageSelection},
    registry::{
        self,
        model::{Entry, Registry, Release},
    },
    state::{InstalledPackage, State},
    step,
};

const SUGGESTION_THRESHOLD: f64 = 0.7;
const MAX_SUGGESTIONS: usize = 3;

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

pub fn accepted_action(pkgs: &[Entry], yes: bool, action: &Action, state: &State) -> bool {
    header!(
        "Packages to be {} ({}):\n",
        action.past_participle(),
        pkgs.len()
    );

    list_entries(pkgs, |e| {
        action.should_skip(state, e).then(|| action.marker())
    });

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
        let suggestions = suggest_similar(m, suggest_pool.iter().map(String::as_str));

        if suggestions.is_empty() {
            end_error!("Package {} doesn't exist.", m.quote());
        } else {
            let list = suggestions
                .iter()
                .map(|s| s.quote().to_string())
                .collect::<Vec<_>>()
                .join(", ");

            end_error!("Package {} doesn't exist. Did you mean: {list}?", m.quote());
        }
    }

    Err(())
}

pub const fn plural<'a>(count: i32, singular: &'a str, plural: &'a str) -> &'a str {
    if count == 1 { singular } else { plural }
}

pub fn write_entries(
    entries: &[Entry],
    verbose: bool,
    installed_packages: &HashMap<String, InstalledPackage>,
    show_marker: bool,
) {
    let installed_version = |e: &Entry| {
        installed_packages
            .get(&e.name)
            .map(|pkg| pkg.version.clone())
    };

    if verbose {
        for entry in entries {
            entry.print_detailed(installed_version(entry));
        }
    } else {
        list_entries(entries, |e| {
            (show_marker && installed_packages.contains_key(&e.name)).then_some(Marker::Installed)
        });
    }
}

pub fn list_entries(entries: &[Entry], marker: impl Fn(&Entry) -> Option<Marker>) {
    let name_width = entries
        .iter()
        .map(|e| e.name.len())
        .max()
        .unwrap_or(0)
        .max(15);

    let version_width = entries
        .iter()
        .map(|e| e.source.purl.version.len())
        .max()
        .unwrap_or(0)
        .max(15);

    let source_width = entries
        .iter()
        .map(|e| e.source.purl.kind.to_string().len())
        .max()
        .unwrap_or(0)
        .max(6);

    println!(
        "{:<name_width$}  {:<version_width$}  {}",
        "Name".bold(),
        "Version".bold(),
        "Source".bold(),
        name_width = name_width,
        version_width = version_width,
    );

    println!(
        "{}",
        "─"
            .repeat(name_width + version_width + source_width + 4)
            .dimmed()
    );

    for entry in entries {
        let name = if entry.deprecation.is_some() {
            entry.name.strikethrough().dimmed().to_string()
        } else {
            entry.name.clone()
        };

        let label = marker(entry)
            .map(|m| format!("  {}", m.render()))
            .unwrap_or_default();

        println!(
            "{name}{}  {:<version_width$}  {:<source_width$}{label}",
            " ".repeat(name_width.saturating_sub(entry.name.len())),
            entry.source.purl.version.cyan(),
            entry.source.purl.kind.to_string().dimmed(),
            version_width = version_width,
            source_width = source_width,
        );
    }
}

pub fn list_release_tags(tags: &[String], current: &str, page: u32) {
    let name_width = tags.iter().map(|t| t.len()).max().unwrap_or(0).max(15);

    println!("{:<name_width$}", "Tag".bold());
    println!("{}", "─".repeat(name_width).dimmed());

    for (i, tag) in tags.iter().enumerate() {
        let is_latest = page <= 1 && i == 0;
        let is_installed = tag == current;

        let padding = " ".repeat(name_width.saturating_sub(tag.len()));

        let (colored_tag, markers) = match (is_latest, is_installed) {
            (true, true) => (
                tag.green().to_string(),
                format!("{} {}", "(latest)".cyan(), "(installed)".green()),
            ),
            (true, false) => (tag.cyan().to_string(), "(latest)".cyan().to_string()),
            (false, true) => (tag.green().to_string(), "(installed)".green().to_string()),
            (false, false) => (tag.clone(), String::new()),
        };

        println!("{colored_tag}  {padding}{markers}");
    }
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

pub fn accepted_sync(entries: &[Entry], state: &State, yes: bool) -> bool {
    header!(
        "Packages to be {} ({}):\n",
        Action::Sync.past_participle(),
        entries.len()
    );

    let rows: Vec<_> = entries
        .iter()
        .map(|e| {
            let current = state
                .installed
                .get(&e.name)
                .map(|installed| installed.version.clone())
                .unwrap_or_else(|| "?".into());

            let synced = Action::Sync.should_skip(state, e);

            let new = if synced {
                "-".to_string()
            } else {
                e.source.purl.version.clone()
            };

            (e.name.clone(), current, new, synced)
        })
        .collect();

    let name_w = rows
        .iter()
        .map(|(n, _, _, _)| n.len())
        .max()
        .unwrap_or(0)
        .max(15);

    let cur_w = rows
        .iter()
        .map(|(_, c, _, _)| c.len())
        .max()
        .unwrap_or(0)
        .max(15);

    let new_w = rows
        .iter()
        .map(|(_, _, n, _)| n.len())
        .max()
        .unwrap_or(0)
        .max(6);

    let pad = |s: &str, w: usize| format!("{s}{}", " ".repeat(w.saturating_sub(s.len())));

    println!(
        "{}  {}  {}",
        pad("Name", name_w).bold(),
        pad("Current", cur_w).bold(),
        "New".bold(),
    );
    println!("{}", "─".repeat(name_w + cur_w + new_w + 4).dimmed());

    for (name, current, new, synced) in &rows {
        let marker = if *synced {
            " (synced)".cyan().to_string()
        } else {
            String::new()
        };

        let new_cell = if *synced {
            pad(new, new_w).dimmed().to_string()
        } else {
            pad(new, new_w).green().to_string()
        };

        println!(
            "{}  {}  {}{}",
            pad(name, name_w),
            pad(current, cur_w).cyan(),
            new_cell,
            marker,
        );
    }

    println!();
    confirm_action(&format!("Proceed with {}?", Action::Sync.noun()), yes)
}

pub fn latest_marker(is_latest: bool) -> String {
    if is_latest {
        " (latest)".cyan().to_string()
    } else {
        String::new()
    }
}

fn installed_marker(is_installed: bool) -> String {
    if is_installed {
        " (installed)".green().to_string()
    } else {
        String::new()
    }
}

pub fn installed_names(state: &State) -> Vec<String> {
    state.installed.keys().cloned().collect()
}

pub fn resolve_selection(state: &State, selection: PackageSelection) -> Vec<String> {
    match selection {
        PackageSelection::Specific(items) => items,
        PackageSelection::All => installed_names(state),
    }
}

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
        "Version to be set: {}{}{}",
        release.tag.quote(),
        latest_marker(is_latest),
        installed_marker(is_installed)
    );

    if is_installed {
        note!(
            "{} The already installed registry is the same as the one you are going to install.",
            "[!]".yellow()
        );
    }
}

pub fn commit_registry(release: &Release, bytes: &[u8], state: &mut State) -> OperationResult {
    if let Err(e) = registry::write_registry_bytes(bytes) {
        end_error!("Couldn't save new registry: {e}");
        return OperationResult::Failure;
    }

    state.set_registry_tag(release.tag.clone());

    if let Err(e) = state.save() {
        end_error!("Couldn't save updated state: {e}");
        return OperationResult::Failure;
    }

    end!("Registry version set successfully.");
    OperationResult::Success
}
