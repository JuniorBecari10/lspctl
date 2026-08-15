use std::{fs::File, io::Read, path::PathBuf};

use const_format::concatcp;
use zip::ZipArchive;

use crate::{def_consts, disk, paths};

def_consts!(
    REGISTRY_FILE = "registry.json",
    REGISTRY_ZIP = concatcp!(REGISTRY_FILE, ".zip"),
);

pub fn write_registry_to_disk(data: &[u8]) -> anyhow::Result<()> {
    disk::write_file_atomic_contents(&get_registry_path(), data, true)
}

pub fn get_registry_path() -> PathBuf {
    paths::registry_dir().join(REGISTRY_FILE)
}
