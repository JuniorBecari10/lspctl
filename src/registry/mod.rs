use std::{fs::File, io::Read};

use anyhow::anyhow;
use const_format::formatcp;
use serde::de::DeserializeOwned;

use crate::{
    disk,
    log::Format,
    note, paths,
    registry::model::{RawRegistry, Registry, Release, ReleaseAsset},
    step,
};

pub mod model;
pub mod parser;
pub mod resolver;
mod util;

// Export for other packages to use as well
pub use util::REGISTRY_FILE;

const RELEASES_API_URL: &str = "https://api.github.com/repos/mason-org/mason-registry/releases";

const ITEMS_PER_PAGE: u32 = 15;
const RELEASE_PAGE_URL: &str = formatcp!(
    "https://api.github.com/repos/mason-org/mason-registry/releases?per_page={ITEMS_PER_PAGE}&page="
);

fn release_by_tag_url(tag: &str) -> String {
    format!("{RELEASES_API_URL}/tags/{tag}")
}

fn release_page_url(page: u32) -> String {
    format!("{RELEASE_PAGE_URL}{page}")
}

// ---

fn parse_json<T: DeserializeOwned>(raw: &[u8]) -> anyhow::Result<T> {
    Ok(serde_json::from_slice(raw)?)
}

fn fetch_json<T: DeserializeOwned>(url: &str) -> anyhow::Result<T> {
    let mut raw_data = Vec::new();
    disk::perform_request(url)?.0.read_to_end(&mut raw_data)?;
    parse_json(&raw_data)
}

pub fn fetch_release_page(page: u32) -> anyhow::Result<Vec<Release>> {
    fetch_json(&release_page_url(page))
}

pub fn fetch_latest_release() -> anyhow::Result<Release> {
    fetch_release_page(1)?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("Couldn't get latest release"))
}

pub fn fetch_release(version: &str) -> anyhow::Result<Release> {
    if version == "latest" {
        return fetch_latest_release();
    }

    fetch_json(&release_by_tag_url(version))
}

fn find_registry_asset(release: &Release) -> anyhow::Result<&ReleaseAsset> {
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

pub fn download_latest_registry() -> anyhow::Result<String> {
    step!("Fetching latest registry...");

    let release = fetch_latest_release()?;
    install_registry_from_release(&release)?;

    note!("Fetching complete.");

    Ok(release.tag)
}

pub fn fetch_registry_bytes_from_release(release: &Release) -> anyhow::Result<Vec<u8>> {
    let asset = find_registry_asset(release)?;

    let mut zip = disk::new_temp()?;
    let zip_file = zip.as_file_mut();

    disk::download_file(&asset.url, zip_file)?;
    disk::extract_to_memory(zip_file, REGISTRY_FILE)
}

pub fn install_registry_from_release(release: &Release) -> anyhow::Result<()> {
    let extracted = fetch_registry_bytes_from_release(release)?;
    write_registry_bytes(&extracted)
}

pub fn write_registry_bytes(bytes: &[u8]) -> anyhow::Result<()> {
    util::write_registry_to_disk(bytes)
}

pub fn parse_registry_from_bytes(bytes: &[u8]) -> anyhow::Result<Registry> {
    let raw: RawRegistry = parse_json(bytes)?;
    parser::parse_registry(raw)
}

pub fn read_registry() -> anyhow::Result<Registry> {
    let mut contents = Vec::new();
    File::open(paths::registry_file())?.read_to_end(&mut contents)?;
    parse_registry_from_bytes(&contents)
}
