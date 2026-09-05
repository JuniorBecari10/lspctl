use std::fs;

use crate::{disk, error, paths, registry};

pub fn setup_root() -> anyhow::Result<()> {
    ensure_root_items()?;

    match paths::registry_file().try_exists() {
        Ok(true) => Ok(()),

        // covers Ok(false) and Err(_)
        _ => registry::download_registry(),
    }
}

// TODO: delete all folders in packages that isn't in registry.
// basically the recover/clean command. make this a manual operation.
fn ensure_root_items() -> anyhow::Result<()> {
    // clean tmp dir, which also deletes the directory. but it's created again below
    clean_tmp();

    fs::create_dir_all(paths::bin_dir())?;
    fs::create_dir_all(paths::tmp_dir())?;
    fs::create_dir_all(paths::registry_dir())?;
    fs::create_dir_all(paths::packages_dir())?;

    Ok(())
}

// this only prints errors, and doesn't block the command's job
fn clean_tmp() {
    let dir = paths::tmp_dir();

    if !dir.exists() {
        return; // nothing to clean
    }

    if let Err(e) = disk::make_writable_recursive(&dir) {
        error!("Failed to prepare tmp directory for cleanup: {e}");
        return;
    }

    if let Err(e) = fs::remove_dir_all(&dir) {
        error!("Failed to clean tmp directory: {e}");
    }
}
