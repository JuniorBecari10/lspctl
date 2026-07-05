use std::fs::create_dir_all;

use crate::{folders, registry};

pub fn setup_root() -> anyhow::Result<()> {
    ensure_folders()?;
    registry::download_registry()?;

    Ok(())
}

fn ensure_folders() -> anyhow::Result<()> {
    create_dir_all(folders::bin_dir())?;
    create_dir_all(folders::registry_dir())?;
    create_dir_all(folders::packages_dir())?;

    Ok(())
}
