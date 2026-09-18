use std::{fmt::Display, process::ExitCode};

use colored::Colorize;
use regex::Regex;

use crate::{operations::model::SearchFilter, registry::model::Entry, state::State};

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
    Sync,
}

impl Action {
    pub const fn verb_base(&self) -> &'static str {
        match self {
            Action::Install => "install",
            Action::Remove => "remove",
            Action::Sync => "sync",
        }
    }

    pub const fn gerund(&self) -> &'static str {
        match self {
            Action::Install => "Installing",
            Action::Remove => "Removing",
            Action::Sync => "Syncing",
        }
    }

    pub const fn past_participle(&self) -> &'static str {
        match self {
            Action::Install => "installed",
            Action::Remove => "removed",
            Action::Sync => "synced",
        }
    }

    pub const fn noun(&self) -> &'static str {
        match self {
            Action::Install => "installation",
            Action::Remove => "removal",
            Action::Sync => "sync",
        }
    }

    // TODO: also check version
    pub fn should_skip(&self, state: &State, entry: &Entry) -> bool {
        match self {
            Action::Install => state.package_exists(&entry.name),
            Action::Remove => !state.package_exists(&entry.name),
            Action::Sync => state
                .installed
                .get(&entry.name)
                .is_some_and(|installed| installed.version == entry.source.purl.version),
        }
    }

    pub const fn skip_reason(&self) -> &'static str {
        match self {
            Action::Install => "is already installed",
            Action::Remove => "is already not installed",
            Action::Sync => "is already synced",
        }
    }

    pub const fn skip_tally_word(&self) -> &'static str {
        match self {
            Action::Install => "already installed",
            Action::Remove => "already not installed",
            Action::Sync => "already synced",
        }
    }

    pub const fn marker(&self) -> Marker {
        match self {
            Action::Install => Marker::Installed,
            Action::Remove => Marker::NotInstalled,
            Action::Sync => Marker::Matches,
        }
    }
}

pub enum Marker {
    Installed,
    NotInstalled,
    Matches,
}

impl Marker {
    pub fn render(&self) -> colored::ColoredString {
        match self {
            Marker::Installed => "(installed)".green(),
            Marker::NotInstalled => "(not installed)".yellow(),
            Marker::Matches => "(synced)".green(),
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
    pub fn matches(&self, entry: &Entry) -> bool {
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
