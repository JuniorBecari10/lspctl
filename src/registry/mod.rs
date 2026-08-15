use std::{fs::File, io::Read};

use anyhow::anyhow;

use crate::{disk, note, paths, registry::model::RawRegistry, step};

pub mod model;
pub mod parser;
pub mod resolver;
mod util;

// Export for other packages to use as well
pub use util::REGISTRY_FILE;

const MASON_URL: &str = "https://api.github.com/repos/mason-org/mason-registry/releases/latest";

fn get_latest_release() -> anyhow::Result<()> {
    let mut raw_data = Vec::new();
    disk::perform_request(MASON_URL)?.read_to_end(&mut raw_data)?;

    let data = parse_release(&raw_data)?;
    let asset = find_registry_asset(&data)?;

    let mut zip = disk::new_temp()?;
    let zip_file = zip.as_file_mut();

    disk::download_file(&asset.url, zip_file)?;

    // TODO: extract the zip with an iterator to the file and write it directly into the final destination
    let extracted = disk::extract_to_memory(zip_file, REGISTRY_FILE)?;
    util::write_registry_to_disk(&extracted)?;

    Ok(())
}

fn find_registry_asset(release: &model::Release) -> anyhow::Result<&model::ReleaseAsset> {
    release
        .assets
        .iter()
        .find(|a| a.name == util::REGISTRY_ZIP) // TODO: fetch 'checksums.txt' as well
        .ok_or_else(|| anyhow!("'{}' not found in release assets.", util::REGISTRY_ZIP))
}

fn parse_release(raw_json: &[u8]) -> anyhow::Result<model::Release> {
    Ok(serde_json::from_slice(raw_json)?)
}

pub fn download_registry() -> anyhow::Result<()> {
    step!("Fetching registry..");
    get_latest_release()?;
    note!("Fetching complete.");

    Ok(())
}

pub fn read_registry() -> anyhow::Result<model::Registry> {
    let mut contents = Vec::new();
    File::open(paths::registry_file())?.read_to_end(&mut contents)?;

    let raw: RawRegistry = serde_json::from_slice(&contents)?;
    parser::parse_registry(raw)
}
