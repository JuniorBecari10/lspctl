use std::fs;

use crate::{paths, registry, step};

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

// TODO: add lockfile to ensure no more than one instance of lspctl runs at once.
// also delete all folders in packages that isn't in registry.
// basically the recover command. maybe make this a manual operation.
// this assumes no other instance is running, so we need to add checks!
fn ensure_root_items() -> anyhow::Result<()> {
    // clean tmp dir. it's created below
    // fs::remove_dir_all(paths::tmp_dir())?;

    fs::create_dir_all(paths::bin_dir())?;
    fs::create_dir_all(paths::tmp_dir())?;
    fs::create_dir_all(paths::registry_dir())?;
    fs::create_dir_all(paths::packages_dir())?;

    Ok(())
}
