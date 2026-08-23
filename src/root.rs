use std::fs;

use crate::{disk, paths, registry, step};

pub fn setup_root() -> anyhow::Result<()> {
    ensure_root_items()?;

    match paths::registry_file().try_exists() {
        Ok(true) => {
            step!("Registry is already downloaded.");
            Ok(())
        }

        // covers Ok(false) and Err(_)
        _ => registry::download_registry(),
    }
}

// TODO: delete all folders in packages that isn't in registry.
// basically the recover command. maybe make this a manual operation.
fn ensure_root_items() -> anyhow::Result<()> {
    // clean tmp dir. it's created below
    clean_tmp();

    fs::create_dir_all(paths::bin_dir())?;
    fs::create_dir_all(paths::tmp_dir())?;
    fs::create_dir_all(paths::registry_dir())?;
    fs::create_dir_all(paths::packages_dir())?;

    Ok(())
}

// this ignores errors because if it doesn't exist, there's nothing to clean.
fn clean_tmp() {
    let _ = disk::make_writable_recursive(&paths::tmp_dir());
    let _ = fs::remove_dir_all(paths::tmp_dir());
}
