use std::{fs::File, io::Read};

use anyhow::anyhow;
use const_format::formatcp;

use crate::{
    disk,
    log::Format,
    note, paths,
    registry::model::{RawRegistry, Release},
    step,
};

pub mod model;
pub mod parser;
pub mod resolver;
mod util;

// Export for other packages to use as well
pub use util::REGISTRY_FILE;

const REGISTRY_URL: &str = "https://api.github.com/repos/mason-org/mason-registry/releases";

const ITEMS_PER_PAGE: u32 = 10;
const REGISTRY_LIST_URL: &str = formatcp!(
    "https://api.github.com/repos/mason-org/mason-registry/releases?per_page={ITEMS_PER_PAGE}&page="
);

fn registry_url(version: &str) -> String {
    if version == "latest" {
        format!("{REGISTRY_URL}/{version}")
    } else {
        format!("{REGISTRY_URL}/tags/{version}")
    }
}

fn release_list_url(page: u32) -> String {
    format!("{REGISTRY_LIST_URL}{page}")
}

// ---

pub fn get_release_list(page: u32) -> anyhow::Result<Vec<Release>> {
    let mut raw_data = Vec::new();

    disk::perform_request(&&release_list_url(page))?
        .0
        .read_to_end(&mut raw_data)?;

    Ok(serde_json::from_slice(&raw_data)?)
}

pub fn get_release_data(version: &str) -> anyhow::Result<model::Release> {
    let mut raw_data = Vec::new();

    disk::perform_request(&registry_url(version))?
        .0
        .read_to_end(&mut raw_data)?;

    parse_release(&raw_data)
}

pub fn get_registry_release(version: &str) -> anyhow::Result<String> {
    let data = get_release_data(version)?;
    let asset = find_registry_asset(&data)?;

    let mut zip = disk::new_temp()?;
    let zip_file = zip.as_file_mut();

    disk::download_file(&asset.url, zip_file)?;

    // TODO: extract the zip with an iterator to the file and write it directly into the final destination
    let extracted = disk::extract_to_memory(zip_file, REGISTRY_FILE)?;
    util::write_registry_to_disk(&extracted)?;

    Ok(data.tag_name)
}

fn get_registry_latest_release() -> anyhow::Result<String> {
    get_registry_release("latest")
}

fn find_registry_asset(release: &model::Release) -> anyhow::Result<&model::ReleaseAsset> {
    release
        .assets
        .iter()
        .find(|a| a.name == util::REGISTRY_ZIP) // TODO: fetch 'checksums.txt' as well
        .ok_or_else(|| {
            anyhow!(
                "{} not found in release assets.",
                util::REGISTRY_ZIP.quote()
            )
        })
}

fn parse_release(raw_json: &[u8]) -> anyhow::Result<model::Release> {
    Ok(serde_json::from_slice(raw_json)?)
}

pub fn download_registry() -> anyhow::Result<String> {
    step!("Fetching latest registry...");
    let tag = get_registry_latest_release()?;
    note!("Fetching complete.");

    Ok(tag)
}

pub fn read_registry() -> anyhow::Result<model::Registry> {
    let mut contents = Vec::new();
    File::open(paths::registry_file())?.read_to_end(&mut contents)?;

    let raw: RawRegistry = serde_json::from_slice(&contents)?;
    parser::parse_registry(raw)
}
