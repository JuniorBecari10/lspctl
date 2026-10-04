use std::fs;

use anyhow::Context;

use crate::{disk, log::Format, paths, state::State};

pub fn remove(name: &str, state: &mut State) -> anyhow::Result<()> {
    let state_entry = state
        .get_entry(name)
        .ok_or_else(|| anyhow::anyhow!("Package {} is not installed", name.quote()))?;

    for file in state_entry.bin.values() {
        match fs::remove_file(file) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}

            Err(e) => {
                return Err(e)
                    .with_context(|| format!("Failed to remove link {}", file.display().quote()));
            }
        }
    }

    remove_package(name)?;
    state.remove_entry(name);
    Ok(())
}

fn remove_package(name: &str) -> anyhow::Result<()> {
    let path = paths::package_dir(name);
    disk::make_writable_recursive(&path)?;
    fs::remove_dir_all(&path)?;

    Ok(())
}
