use std::fs;

use crate::paths;

pub mod clean;

pub fn setup_root() -> anyhow::Result<()> {
    clean::clean_tmp();

    fs::create_dir_all(paths::bin_dir())?;
    fs::create_dir_all(paths::tmp_dir())?;
    fs::create_dir_all(paths::registry_dir())?;
    fs::create_dir_all(paths::packages_dir())?;

    Ok(())
}
