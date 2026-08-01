use std::{collections::HashSet, process::ExitCode};

use colored::Colorize;
use dialoguer::Confirm;

use crate::{
    header, list,
    registry::{
        model::{Asset, Entry, Platform, Registry, ResolvedEntry},
        parser::template::{
            self,
            context::{self},
        },
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

pub fn resolve_entry(
    e: Entry,
    platform: &Platform,
) -> anyhow::Result<(ResolvedEntry, Option<Asset>)> {
    let ctx = context::build_context(&e.source, platform)?;

    let entry = template::resolve_entry(e, &ctx)?;
    let asset = ctx.asset.map(serde_json::from_value).transpose()?; // shouldn't fail

    Ok((entry, asset))
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

pub fn accepted_installation(pkgs: &[Entry], yes: bool) -> bool {
    list_installing_packages(pkgs);

    yes || {
        eprintln!();
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

pub fn plural<'a>(var: i32, singular: &'a str, plural: &'a str) -> &'a str {
    if var == 1 { singular } else { plural }
}
