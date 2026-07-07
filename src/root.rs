use std::fs::{File, create_dir_all};

use crate::{folders, registry};

pub fn setup_root() -> anyhow::Result<()> {
    ensure_root_items()?;
    registry::download_registry()?;

    Ok(())
}

fn ensure_root_items() -> anyhow::Result<()> {
    create_dir_all(folders::bin_dir())?;
    create_dir_all(folders::registry_dir())?;
    create_dir_all(folders::packages_dir())?;
    File::create(folders::state_file())?;

    Ok(())
}
