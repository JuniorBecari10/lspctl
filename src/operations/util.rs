use std::{collections::HashSet, process::ExitCode};

use colored::Colorize;
use dialoguer::Confirm;

use crate::{
    header, list,
    registry::{
        model::{Entry, Platform, Registry},
        parser::template::{self, context},
    },
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

pub fn resolve_entry(e: Entry, platform: &Platform) -> anyhow::Result<Entry> {
    let ctx = context::build_context(&e.source, platform)?;
    template::resolve_entry(e, ctx)
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

pub fn prompt_user(pkgs: &[Entry], yes: bool) -> bool {
    list_installing_packages(pkgs);
    println!(); // TODO: change to stderr

    if yes {
        true
    } else {
        Confirm::new()
            .with_prompt(format!(" {} Proceed with installation?", "-".green()))
            .default(true)
            .interact()
            .unwrap_or(false)
    }
}

fn list_installing_packages(pkgs: &[Entry]) {
    header!("Packages to be installed ({}):", pkgs.len());

    for pkg in pkgs {
        list!("{}", pkg);
    }
}
