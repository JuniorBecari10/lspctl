use std::fs;

use anyhow::Context;

use crate::{
    disk,
    log::Format,
    paths,
    registry::model::util,
    state::{Executables, State},
};

pub fn remove(name: &str, state: &mut State) -> anyhow::Result<()> {
    let state_entry = state
        .get_package(name)
        .ok_or_else(|| anyhow::anyhow!("Package {} is not installed", name.quote()))?;

    for file in state_entry.bin.values().flat_map(Executables::iter) {
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
    let name_path = util::sanitize_path_component(name);
    let path = paths::package_dir(&name_path);

    disk::make_writable_recursive(&path)?;
    fs::remove_dir_all(&path)?;

    Ok(())
}
