use std::{collections::HashSet, process::ExitCode};

use colored::Colorize;
use dialoguer::Confirm;

use crate::{
    end, error, header, list,
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
    fn verb_base(&self) -> &'static str {
        match self {
            Action::Install => "install",
            Action::Remove => "remove",
        }
    }

    fn gerund(&self) -> &'static str {
        match self {
            Action::Install => "Installing",
            Action::Remove => "Removing",
        }
    }

    fn past_participle(&self) -> &'static str {
        match self {
            Action::Install => "installed",
            Action::Remove => "removed",
        }
    }

    fn noun(&self) -> &'static str {
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

    fn skip_reason(&self) -> &'static str {
        match self {
            Action::Install => "is already installed",
            Action::Remove => "is already not installed",
        }
    }

    fn skip_tally_word(&self) -> &'static str {
        match self {
            Action::Install => "already installed",
            Action::Remove => "already not installed",
        }
    }
}

fn accepted_action(pkgs: &[Entry], yes: bool, action: &Action) -> bool {
    header!(
        "Packages to be {} ({}):",
        action.past_participle(),
        pkgs.len()
    );

    for pkg in pkgs {
        list!("{}", pkg.format_line());
    }

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
    let (registry, platform, mut state) = prelude::prelude();
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

fn plural<'a>(count: i32, singular: &'a str, plural: &'a str) -> &'a str {
    if count == 1 { singular } else { plural }
}
