use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use const_format::concatcp;
use zip::ZipArchive;

use crate::{consts, def_consts, disk, paths};

def_consts!(
    REGISTRY_FILE = "registry.json",
    REGISTRY_ZIP = concatcp!(REGISTRY_FILE, ".zip"),
);

pub fn download_file(url: &str, dest: &mut File) -> anyhow::Result<()> {
    let data = perform_request(url)?;
    dest.write_all(&data)?;

    Ok(())
}

pub fn extract_to_memory(zip: &File) -> anyhow::Result<Vec<u8>> {
    let mut archive = ZipArchive::new(zip)?;
    let mut entry = archive.by_name(REGISTRY_FILE)?;
    let mut contents = Vec::new();

    entry.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn write_registry_to_disk(data: &[u8]) -> anyhow::Result<()> {
    disk::write_file_atomic_contents(&get_registry_path(), data, true)
}

pub fn perform_request(url: &str) -> anyhow::Result<Vec<u8>> {
    if !url.starts_with("https://") {
        anyhow::bail!("This only performs https requests. URL: '{url}'.");
    }

    Ok(ureq::get(url)
        .header("User-Agent", consts::APP_NAME)
        .call()?
        .body_mut()
        .read_to_vec()?)
}

pub fn get_registry_path() -> PathBuf {
    paths::registry_dir().join(REGISTRY_FILE)
}
