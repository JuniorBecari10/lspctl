use std::{collections::HashSet, process::ExitCode};

use crate::registry::model::{Entry, Registry};

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

pub fn filter_registry<'a>(
    registry: &'a Registry,
    pkgs: &'a [String],
) -> (Vec<&'a Entry>, Vec<&'a str>) {
    let wanted: HashSet<&str> = pkgs.iter().map(String::as_str).collect();

    let found: Vec<&Entry> = registry
        .0
        .iter()
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
