use std::{io::Read, path::PathBuf};

use anyhow::anyhow;
use serde::de::DeserializeOwned;

use crate::{
    disk,
    log::Format,
    paths,
    registry::{
        consts,
        model::{Release, ReleaseAsset},
    },
};

pub fn get_registry_path() -> PathBuf {
    paths::registry_dir().join(consts::REGISTRY_FILE)
}

// ---

pub fn release_by_tag_url(tag: &str) -> String {
    format!("{}/tags/{tag}", consts::RELEASES_API_URL)
}

pub fn release_page_url(page: u32) -> String {
    format!("{}{page}", consts::RELEASE_PAGE_URL)
}

// ---

// this is used because it automatically converts the result into anyhow
pub fn parse_json<T: DeserializeOwned>(raw: &[u8]) -> anyhow::Result<T> {
    Ok(serde_json::from_slice(raw)?)
}

pub fn fetch_json<T: DeserializeOwned>(url: &str) -> anyhow::Result<T> {
    let mut raw_data = Vec::new();
    disk::perform_request(url)?.0.read_to_end(&mut raw_data)?;
    parse_json(&raw_data)
}

pub fn find_registry_asset(release: &Release) -> anyhow::Result<&ReleaseAsset> {
    release
        .assets
        .iter()
        .find(|a| a.name == consts::REGISTRY_ZIP) // TODO: fetch 'checksums.txt' as well
        .ok_or_else(|| {
            anyhow!(
                "{} not found in release assets.",
                consts::REGISTRY_ZIP.quote()
            )
        })
}
