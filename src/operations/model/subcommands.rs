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
}

impl Action {
    pub const fn verb_base(&self) -> &'static str {
        match self {
            Action::Install => "install",
            Action::Remove => "remove",
        }
    }

    pub const fn gerund(&self) -> &'static str {
        match self {
            Action::Install => "Installing",
            Action::Remove => "Removing",
        }
    }

    pub const fn past_participle(&self) -> &'static str {
        match self {
            Action::Install => "installed",
            Action::Remove => "removed",
        }
    }

    pub const fn noun(&self) -> &'static str {
        match self {
            Action::Install => "installation",
            Action::Remove => "removal",
        }
    }

    pub fn should_skip(&self, state: &State, name: &str) -> bool {
        match self {
            Action::Install => state.package_exists(name),
            Action::Remove => !state.package_exists(name),
        }
    }

    pub const fn skip_reason(&self) -> &'static str {
        match self {
            Action::Install => "is already installed",
            Action::Remove => "is already not installed",
        }
    }

    pub const fn skip_tally_word(&self) -> &'static str {
        match self {
            Action::Install => "already installed",
            Action::Remove => "already not installed",
        }
    }

    pub const fn marker(&self) -> Marker {
        match self {
            Action::Install => Marker::Installed,
            Action::Remove => Marker::NotInstalled,
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
