use std::fs::{File, create_dir_all};

use crate::{folders, registry};

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
    create_dir_all(folders::bin_dir())?;
    create_dir_all(folders::registry_dir())?;
    create_dir_all(folders::packages_dir())?;
    File::create(folders::state_file())?;

    Ok(())
}
