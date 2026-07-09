use std::{fs::File, io::Read};

use anyhow::anyhow;

use crate::folders;

mod model;
mod util;

// Export for other packages to use as well
pub use util::REG_NAME;

const MASON_URL: &str = "https://api.github.com/repos/mason-org/mason-registry/releases/latest";

fn get_latest_release() -> anyhow::Result<()> {
    let data = parse_release(&util::perform_request(MASON_URL)?)?;
    let asset = find_registry_asset(&data)?;

    let mut temp_zip = tempfile::tempfile()?;
    util::download_file(&asset.url, &mut temp_zip)?;

    // maybe use this to pass the json around to not read it again
    let extracted = util::extract_to_memory(&temp_zip)?;
    util::write_registry_to_disk(&extracted)?;

    Ok(())
}

fn find_registry_asset(release: &model::Release) -> anyhow::Result<&model::Asset> {
    release
        .assets
        .iter()
        .find(|a| a.name == util::REG_ZIP_NAME) // TODO: fetch 'checksums.txt' as well
        .ok_or_else(|| anyhow!("'{}' not found in release assets.", util::REG_ZIP_NAME))
}

fn parse_release(raw_json: &[u8]) -> anyhow::Result<model::Release> {
    Ok(serde_json::from_slice(raw_json)?)
}

pub fn download_registry() -> anyhow::Result<()> {
    log::info!("Fetching registry...");
    get_latest_release()?;
    log::info!("Fetching complete.");

    Ok(())
}

pub fn read_registry() -> anyhow::Result<model::Registry> {
    let mut contents = Vec::new();

    File::open(folders::registry_file())?.read_to_end(&mut contents)?;
    Ok(serde_json::from_slice(&contents)?)
}
