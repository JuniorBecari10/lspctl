use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    process::ExitCode,
};

use colored::Colorize;
use dialoguer::Confirm;
use regex::Regex;

use crate::{
    end, error, header,
    log::Fatal,
    note,
    operations::prelude,
    registry::model::{Entry, Platform, Registry},
    state::{InstalledPackage, State},
    step,
};

pub enum OperationResult {
    Success,
    Failure,
}

pub enum PackageSelection {
    Specific(Vec<String>),
    All,
}

impl From<OperationResult> for ExitCode {
    fn from(res: OperationResult) -> Self {
        match res {
            OperationResult::Success => Self::SUCCESS,
            OperationResult::Failure => Self::FAILURE,
        }
    }
}

pub enum Action {
    Install,
    Remove,
}

impl Action {
    const fn verb_base(&self) -> &'static str {
        match self {
            Action::Install => "install",
            Action::Remove => "remove",
        }
    }

    const fn gerund(&self) -> &'static str {
        match self {
            Action::Install => "Installing",
            Action::Remove => "Removing",
        }
    }

    const fn past_participle(&self) -> &'static str {
        match self {
            Action::Install => "installed",
            Action::Remove => "removed",
        }
    }

    const fn noun(&self) -> &'static str {
        match self {
            Action::Install => "installation",
            Action::Remove => "removal",
        }
    }

    fn should_skip(&self, state: &State, name: &str) -> bool {
        match self {
            Action::Install => state.package_exists(name),
            Action::Remove => !state.package_exists(name),
        }
    }

    const fn skip_reason(&self) -> &'static str {
        match self {
            Action::Install => "is already installed",
            Action::Remove => "is already not installed",
        }
    }

    const fn skip_tally_word(&self) -> &'static str {
        match self {
            Action::Install => "already installed",
            Action::Remove => "already not installed",
        }
    }

    const fn marker(&self) -> Marker {
        match self {
            Action::Install => Marker::Installed,
            Action::Remove => Marker::NotInstalled,
        }
    }
}

enum Marker {
    Installed,
    NotInstalled,
}

impl Marker {
    fn render(&self) -> colored::ColoredString {
        match self {
            Marker::Installed => "(installed)".green(),
            Marker::NotInstalled => "(not installed)".yellow(),
        }
    }
}

fn accepted_action(pkgs: &[Entry], yes: bool, action: &Action, state: &State) -> bool {
    header!(
        "Packages to be {} ({}):\n",
        action.past_participle(),
        pkgs.len()
    );

    print_entries(pkgs, |e| {
        action.should_skip(state, &e.name).then(|| action.marker())
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

    let (entries, missing) = filter_registry(registry, &pkgs);
    if !missing.is_empty() {
        for m in missing {
            error!("Package '{m}' doesn't exist.");
        }
        return OperationResult::Failure;
    }

    if !accepted_action(&entries, yes, &action, &state) {
        return OperationResult::Success;
    }

    let (mut ok_count, mut err_count, mut skip_count) = (0, 0, 0);

    for pkg in entries {
        if action.should_skip(&state, &pkg.name) {
            step!(
                "Package '{}' {}. Skipping..",
                pkg.name,
                action.skip_reason()
            );
            skip_count += 1;
            continue;
        }

        let name = pkg.name.clone();
        step!("{} package '{name}'..", action.gerund());

        match op(pkg, &platform, &mut state) {
            Ok(()) => {
                end!("Package {} successfully.", action.past_participle());
                ok_count += 1;
            }
            Err(e) => {
                error!("Failed to {} '{name}': {e}", action.verb_base());
                err_count += 1;
            }
        }
    }

    let ok_plural = plural(ok_count, "package", "packages");
    let skip_plural = plural(skip_count, "was", "were");

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

const fn plural<'a>(count: i32, singular: &'a str, plural: &'a str) -> &'a str {
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
        print_entries(entries, |e| {
            (show_marker && installed_packages.contains_key(&e.name)).then_some(Marker::Installed)
        });
    }
}

pub fn list_packages(installed: bool, verbose: bool, pattern: Option<&Regex>) -> OperationResult {
    let (registry, _, state, _lock) = prelude::prelude();

    let entries: Vec<Entry> = if installed {
        let keys = state.installed.keys().cloned().collect::<Vec<_>>();
        let (found, missing) = filter_registry(registry, keys.as_slice());

        if !missing.is_empty() {
            for m in missing {
                error!("Package '{m}' doesn't exist.");
            }
            return OperationResult::Failure;
        }

        found
    } else {
        registry.0
    };

    let entries: Vec<Entry> = match pattern {
        Some(re) => entries
            .into_iter()
            .filter(|e| re.is_match(&e.name))
            .collect(),
        None => entries,
    };

    if entries.is_empty() {
        let msg = match (installed, pattern) {
            (true, Some(p)) => format!("No installed packages match '{}'.", p.as_str()),
            (true, None) => "There are no packages installed.".to_string(),
            (false, Some(p)) => format!("No packages match '{}'.", p.as_str()),
            (false, None) => "No packages found.".to_string(),
        };

        end!("{msg}");
        return OperationResult::Success;
    }

    let header_text = match (installed, pattern.is_some()) {
        (true, true) => "All matching installed packages:\n",
        (true, false) => "Installed packages:\n",
        (false, true) => "All matching packages:\n",
        (false, false) => "All packages:\n",
    };

    header!("{header_text}");

    write_entries(&entries, verbose, &state.installed, !installed);
    OperationResult::Success
}

fn print_entries(entries: &[Entry], marker: impl Fn(&Entry) -> Option<Marker>) {
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

    println!(
        "{:<name_width$}  {:<version_width$}  {}",
        "Name".bold(),
        "Version".bold(),
        "Source".bold(),
        name_width = name_width,
        version_width = version_width,
    );
    println!("{}", "─".repeat(name_width + version_width + 10).dimmed());

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
            "{name}{}  {:<version_width$}  {:<6}{label}",
            " ".repeat(name_width.saturating_sub(entry.name.len())),
            entry.source.purl.version.cyan(),
            entry.source.purl.kind.to_string().dimmed(),
            version_width = version_width,
        );
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

    if !confirm_action("Proceed?", yes) {
        return OperationResult::Success;
    }

    delete(path).fatal(fatal_msg);
    OperationResult::Success
}
