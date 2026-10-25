use std::{
    fs::File,
    io::{Cursor, Read},
};

use anyhow::anyhow;

use crate::{
    disk, note, paths,
    registry::model::{RawRegistry, Registry, Release},
    step,
};

mod consts;
pub mod model;
pub mod parser;
pub mod resolver;
mod util;

// Export for other packages to use as well
pub use consts::REGISTRY_FILE;

pub fn download_latest_registry() -> anyhow::Result<String> {
    step!("Fetching latest registry...");

    let release = fetch_latest_release()?;
    install_registry_from_release(&release)?;

    note!("Fetching complete.");
    Ok(release.tag)
}

pub fn fetch_release_page(page: u32) -> anyhow::Result<Vec<Release>> {
    util::fetch_json(&util::release_page_url(page))
}

pub fn fetch_latest_release() -> anyhow::Result<Release> {
    fetch_release_page(1)?
        .first()
        .cloned()
        .ok_or_else(|| anyhow!("Cannot get latest release"))
}

/// use "latest" to get the latest version
pub fn fetch_release(version: &str) -> anyhow::Result<Release> {
    if version == "latest" {
        return fetch_latest_release();
    }

    util::fetch_json(&util::release_by_tag_url(version))
}

pub fn fetch_registry_bytes_from_release(release: &Release) -> anyhow::Result<Vec<u8>> {
    let asset = util::find_registry_asset(release)?;
    let mut zip = Vec::new();

    disk::download_file(&asset.url, &mut zip)?;
    disk::extract_entry_from_zip(Cursor::new(zip), consts::REGISTRY_FILE)
}

pub fn install_registry_from_release(release: &Release) -> anyhow::Result<()> {
    let extracted = fetch_registry_bytes_from_release(release)?;
    write_registry_to_disk(&extracted)
}

pub fn write_registry_to_disk(data: &[u8]) -> anyhow::Result<()> {
    disk::write_file_atomic_contents(&util::get_registry_path(), data, true)
}

pub fn parse_registry_from_bytes(bytes: &[u8]) -> anyhow::Result<Registry> {
    let raw: RawRegistry = util::parse_json(bytes)?;
    parser::parse_registry(raw)
}

pub fn read_registry() -> anyhow::Result<Registry> {
    let mut contents = Vec::new();
    File::open(paths::registry_file())?.read_to_end(&mut contents)?;
    parse_registry_from_bytes(&contents)
}
