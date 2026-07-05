use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use const_format::concatcp;
use zip::ZipArchive;

use crate::{consts, folders};

pub const REG_NAME: &str = "registry.json";
pub const REG_ZIP_NAME: &str = concatcp!(REG_NAME, ".zip");

pub fn download_file(url: &str, dest: &mut File) -> anyhow::Result<()> {
    let data = perform_request(url)?;
    dest.write_all(&data)?;

    Ok(())
}

pub fn extract_to_memory(zip: &File) -> anyhow::Result<Vec<u8>> {
    let mut archive = ZipArchive::new(zip)?;
    let mut entry = archive.by_name(REG_NAME)?;
    let mut contents = Vec::new();

    entry.read_to_end(&mut contents)?;
    Ok(contents)
}
pub fn write_registry_to_disk(data: &[u8]) -> anyhow::Result<()> {
    let mut registry = File::create(get_registry_path())?;
    registry.write_all(data)?;
    Ok(())
}

pub fn perform_request(url: &str) -> anyhow::Result<Vec<u8>> {
    Ok(ureq::get(url)
        .header("User-Agent", consts::APP_NAME)
        .call()?
        .body_mut()
        .read_to_vec()?)
}

pub fn get_registry_path() -> PathBuf {
    folders::registry_dir().join(REG_NAME)
}
