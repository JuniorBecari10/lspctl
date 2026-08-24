use std::{collections::HashSet, process::ExitCode};

use colored::Colorize;
use dialoguer::Confirm;
use regex::Regex;

use crate::{
    end, error, header,
    operations::prelude,
    registry::model::{Entry, Platform, Registry},
    state::State,
    step,
};

pub enum OperationResult {
    Success,
    Failure,
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
}

fn accepted_action(pkgs: &[Entry], yes: bool, action: &Action) -> bool {
    header!(
        "Packages to be {} ({}):\n",
        action.past_participle(),
        pkgs.len()
    );

    print_entries(pkgs, |_| false);

    yes || {
        eprintln!();
        Confirm::new()
            .with_prompt(format!(" {} Proceed with {}?", "-".green(), action.noun()))
            .default(true)
            .interact()
            .unwrap_or(false)
    }
}

pub fn run_action(
    pkgs: Vec<String>,
    yes: bool,
    action: Action,
    op: fn(Entry, &Platform, &mut State) -> anyhow::Result<()>,
) -> OperationResult {
    let (registry, platform, mut state, _lock) = prelude::prelude();
    let (entries, missing) = filter_registry(registry, &pkgs);

    if !missing.is_empty() {
        for m in missing {
            error!("Package '{m}' doesn't exist.");
        }
        return OperationResult::Failure;
    }

    if !accepted_action(&entries, yes, &action) {
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
                error!("Failed to {} '{name}': {e}.", action.verb_base());
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

pub fn write_entries(entries: &[Entry], verbose: bool, installed_names: Option<&HashSet<String>>) {
    let is_installed = |e: &Entry| installed_names.is_some_and(|s| s.contains(&e.name));

    if verbose {
        for entry in entries {
            entry.print_detailed(is_installed(entry));
        }
    } else {
        print_entries(entries, is_installed);
    }
}

pub fn list_packages(installed: bool, verbose: bool, pattern: Option<&Regex>) -> OperationResult {
    let (registry, _, state, _lock) = prelude::prelude_no_log();
    let installed_names: HashSet<String> = state.installed.keys().cloned().collect();

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

    // suppress '(installed)' marking for installed-only listings.
    let marker_set = if installed {
        None
    } else {
        Some(&installed_names)
    };

    write_entries(&entries, verbose, marker_set);
    OperationResult::Success
}

fn print_entries(entries: &[Entry], is_installed: impl Fn(&Entry) -> bool) {
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

        let installed = if is_installed(entry) {
            format!("  {}", "(installed)".green())
        } else {
            String::new()
        };

        println!(
            "{name}{}  {:<version_width$}  {:<6}{installed}",
            " ".repeat(name_width.saturating_sub(entry.name.len())),
            entry.source.purl.version.cyan(),
            entry.source.purl.kind.to_string().dimmed(),
            version_width = version_width,
        );
    }
}
