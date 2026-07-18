use std::fs;

use crate::{paths, io, registry};

pub fn setup_root() -> anyhow::Result<()> {
    ensure_root_items()?;

    match paths::registry_file().try_exists() {
        Ok(true) => {
            log::info!("Registry already exists.");
            Ok(())
        }

        // covers Ok(false) and Err(_)
        _ => registry::download_registry(),
    }
}

fn ensure_root_items() -> anyhow::Result<()> {
    fs::create_dir_all(paths::bin_dir())?;
    fs::create_dir_all(paths::tmp_dir())?;
    fs::create_dir_all(paths::registry_dir())?;
    fs::create_dir_all(paths::packages_dir())?;
    io::new_file_atomic(&paths::state_file())?;

    Ok(())
}
