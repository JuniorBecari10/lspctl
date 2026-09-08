use std::{collections::HashSet, fs, path::PathBuf};

use crate::{disk, error, log::Format, paths, registry, state::State};

pub fn setup_root() -> anyhow::Result<()> {
    ensure_root_items()?;

    match paths::registry_file().try_exists() {
        Ok(true) => Ok(()),

        // covers Ok(false) and Err(_)
        _ => registry::download_registry(),
    }
}

fn ensure_root_items() -> anyhow::Result<()> {
    clean_tmp();

    fs::create_dir_all(paths::bin_dir())?;
    fs::create_dir_all(paths::tmp_dir())?;
    fs::create_dir_all(paths::registry_dir())?;
    fs::create_dir_all(paths::packages_dir())?;

    Ok(())
}

// ---

// this only prints errors, and doesn't block the command's job
fn clean_tmp() {
    let dir = paths::tmp_dir();

    match dir.try_exists() {
        Ok(true) => {}       // exists. let's clean it.
        Ok(false) => return, // doesn't exist. nothing to clean.

        Err(e) => {
            error!("Failed to check existence of tmp: {e}");
            return;
        }
    }

    if let Err(e) = disk::make_writable_recursive(&dir) {
        error!("Failed to prepare tmp directory for cleanup: {e}");
        return;
    }

    if let Err(e) = fs::remove_dir_all(&dir) {
        error!("Failed to clean tmp directory: {e}");
    }
}

pub fn clean_orphans(state: &State) {
    clean_orphan_packages(state);
    clean_orphan_shims(state);
}

fn clean_orphan_packages(state: &State) {
    let entries = match fs::read_dir(paths::packages_dir()) {
        Ok(e) => e,

        Err(e) => {
            error!("Failed to read packages directory: {e}");
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                error!("Failed to read a packages directory entry: {e}");
                continue;
            }
        };

        let path = entry.path();
        if !path.is_dir() {
            continue; // 'packages/' should only ever hold package directories
        }

        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            error!(
                "Skipping package directory with invalid name: {}",
                path.display().quote()
            );

            continue;
        };

        if state.installed.contains_key(name) {
            continue; // still a real, tracked package
        }

        if let Err(e) = disk::make_writable_recursive(&path) {
            error!(
                "Failed to prepare orphaned package {} for cleanup: {e}",
                name.quote()
            );
            continue;
        }

        if let Err(e) = fs::remove_dir_all(&path) {
            error!(
                "Failed to remove orphaned package directory {}: {e}",
                name.quote()
            );
        }
    }
}

fn clean_orphan_shims(state: &State) {
    let valid: HashSet<PathBuf> = state
        .installed
        .values()
        .flat_map(|pkg| pkg.bin.values().cloned())
        .collect();

    let entries = match fs::read_dir(paths::bin_dir()) {
        Ok(e) => e,
        Err(e) => {
            error!("Failed to read bin directory: {e}");
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                error!("Failed to read a bin directory entry: {e}");
                continue;
            }
        };

        let path = entry.path();
        if valid.contains(&path) {
            continue;
        }

        if let Err(e) = fs::remove_file(&path) {
            error!(
                "Failed to remove orphaned shim {}: {e}",
                path.display().quote()
            );
        }
    }
}
