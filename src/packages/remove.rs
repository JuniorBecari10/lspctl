use std::fs;

use anyhow::Context;

use crate::{disk, paths, registry::model::ResolvedEntry, state::State};

pub fn remove(entry: &ResolvedEntry, state: &mut State) -> anyhow::Result<()> {
    let state_entry = state
        .get_entry(&entry.name)
        .ok_or_else(|| anyhow::anyhow!("Package '{}' is not installed", entry.name))?;

    for file in state_entry.bin.values() {
        match fs::remove_file(file) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}

            Err(e) => {
                return Err(e)
                    .with_context(|| format!("Failed to remove link '{}'", file.display()));
            }
        }
    }

    remove_package(&entry.name)?;
    state.remove_entry(&entry.name);
    Ok(())
}

fn remove_package(name: &str) -> anyhow::Result<()> {
    let path = paths::package_dir(name);
    disk::make_writable_recursive(&path)?;
    fs::remove_dir_all(&path)?;

    Ok(())
}
