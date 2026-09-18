use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
    path::Path,
    process::ExitCode,
};

use colored::Colorize;
use dialoguer::Confirm;
use regex::Regex;

use crate::{
    end, end_error, error, header,
    log::{self, Fatal, Format},
    note,
    operations::{
        logic,
        model::{self, Action, Marker, PackageSelection, SearchFilter},
        prelude,
    },
    registry::model::{Entry, Platform, Registry},
    state::{InstalledPackage, State},
    step,
};

const SUGGESTION_THRESHOLD: f64 = 0.7;
const MAX_SUGGESTIONS: usize = 3;

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

fn filter_registry(registry: Registry, pkgs: &[String]) -> (Vec<Entry>, Vec<&str>) {
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
