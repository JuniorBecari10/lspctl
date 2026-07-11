use std::fs;

use crate::{folders, io, registry};

pub fn setup_root() -> anyhow::Result<()> {
    ensure_root_items()?;

    match folders::registry_file().try_exists() {
        Ok(true) => {
            log::info!("Registry already exists.");
            Ok(())
        }

        // covers Ok(false) and Err(_)
        _ => registry::download_registry(),
    }
}

fn ensure_root_items() -> anyhow::Result<()> {
    fs::create_dir_all(folders::bin_dir())?;
    fs::create_dir_all(folders::registry_dir())?;
    fs::create_dir_all(folders::packages_dir())?;
    io::new_file_atomic(&folders::state_file())?;

    Ok(())
}
