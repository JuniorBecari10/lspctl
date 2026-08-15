use std::{fs, path::Path};

use crate::{disk, paths, registry::model::ResolvedEntry, state::State};

pub fn remove(entry: ResolvedEntry, state: &mut State) -> anyhow::Result<()> {
    let state_entry = state
        .get_entry(&entry.name)
        .ok_or_else(|| anyhow::anyhow!("Package '{}' is not installed", entry.name))?;

    for file in state_entry.bin.values() {
        fs::remove_file(file)?;
    }

    remove_package(&entry.name, &entry.source.purl.version)?;
    state.remove_entry(&entry.name);

    Ok(())
}

fn remove_package(name: &str, version: &str) -> anyhow::Result<()> {
    let path = paths::package_dir(name, version);
    disk::make_writable_recursive(&path)?;
    fs::remove_dir_all(&path)?;

    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Package folder should have a parent"))?;

    if is_dir_empty(parent)? {
        disk::make_writable_recursive(parent)?;
        fs::remove_dir_all(parent)?;
    }

    Ok(())
}

fn is_dir_empty(p: &Path) -> anyhow::Result<bool> {
    Ok(p.read_dir()?.next().is_none())
}
