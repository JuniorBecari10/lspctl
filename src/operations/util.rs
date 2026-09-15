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
use strsim::jaro_winkler;

use crate::{
    end, end_error, error, header,
    log::{self, Fatal, Format},
    note,
    operations::{
        logic,
        model::{self, SearchFilter},
        prelude,
    },
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
    Matches,
}

impl Marker {
    fn render(&self) -> colored::ColoredString {
        match self {
            Marker::Installed => "(installed)".green(),
            Marker::NotInstalled => "(not installed)".yellow(),
            Marker::Matches => "(matches registry)".green(),
        }
    }
}

impl Display for SearchFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SearchFilter::*;
        match self {
            Name => write!(f, "name"),
            Version => write!(f, "version"),
            Description => write!(f, "description"),
            License => write!(f, "license"),
            Language => write!(f, "language"),
            Category => write!(f, "category"),
            Source => write!(f, "source"),
        }
    }
}

pub struct SearchQuery {
    pub pattern: Regex,
    pub filters: Vec<SearchFilter>,
}

impl SearchQuery {
    fn matches(&self, entry: &Entry) -> bool {
        let active: &[SearchFilter] = if self.filters.is_empty() {
            &[SearchFilter::Name] // only names; default behavior
        } else {
            &self.filters
        };

        active.iter().any(|f| match f {
            SearchFilter::Name => self.pattern.is_match(&entry.name),
            SearchFilter::Version => self.pattern.is_match(&entry.source.purl.version),
            SearchFilter::Description => self.pattern.is_match(&entry.description),
            SearchFilter::License => entry.licenses.iter().any(|l| self.pattern.is_match(l)),
            SearchFilter::Language => entry.languages.iter().any(|l| self.pattern.is_match(l)),
            SearchFilter::Category => entry.categories.iter().any(|c| self.pattern.is_match(c)),
            SearchFilter::Source => self.pattern.is_match(&entry.source.purl.kind.to_string()),
        })
    }
}

const SUGGESTION_THRESHOLD: f64 = 0.7;
const MAX_SUGGESTIONS: usize = 3;

fn suggest_similar<'a>(name: &str, pool: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
    let mut scored: Vec<(f64, &str)> = pool
        .map(|candidate| (jaro_winkler(name, candidate), candidate))
        .filter(|(score, _)| *score >= SUGGESTION_THRESHOLD)
        .collect();

    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    scored
        .into_iter()
        .take(MAX_SUGGESTIONS)
        .map(|(_, n)| n)
        .collect()
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

    let installed_pool: Vec<_> = match action {
        Action::Install => registry.0.iter().map(|e| e.name.clone()).collect(),
        Action::Remove => state.installed.keys().cloned().collect(),
    };

    let Ok(entries) = filter_print(registry, &pkgs, &installed_pool) else {
        return OperationResult::Failure;
    };

    if !accepted_action(&entries, yes, &action, &state) {
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

pub fn filter_print(
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

pub fn list_packages(
    installed: bool,
    verbose: bool,
    query: Option<SearchQuery>,
) -> OperationResult {
    let (registry, _, state, _lock) = prelude::prelude();

    let entries: Vec<Entry> = if installed {
        let keys = state.installed.keys().cloned().collect::<Vec<_>>();
        let pool: Vec<_> = registry.0.iter().map(|e| e.name.clone()).collect();

        let Ok(found) = filter_print(registry, keys.as_slice(), &pool) else {
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

    write_entries(&entries, verbose, &state.installed, !installed);
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

    let Ok(entries) = filter_print(registry, &pkgs, &pkgs) else {
        return OperationResult::Failure;
    };

    header!("List of packages to sync ({}):", entries.len());

    print_entries(&entries, |e| {
        let outdated = state
            .installed
            .get(&e.name)
            .is_some_and(|installed| installed.version == e.source.purl.version);
        outdated.then_some(Marker::Matches)
    });

    if !confirm_action("Proceed with sync?", yes) {
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

    let ok_plural = plural(ok_count, "package", "packages");
    header!(
        "Successfully synced {ok_count} {ok_plural}. {err_count} had errors. {skip_count} already up to date."
    );

    if err_count == 0 {
        OperationResult::Success
    } else {
        OperationResult::Failure
    }
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
