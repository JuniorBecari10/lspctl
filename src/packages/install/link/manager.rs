use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{disk, paths};

fn link(bins: &[&str], bin_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    let files = disk::list_files(bin_path)?;
    let mut map = HashMap::new();

    for file in files {
        let name = file
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or_else(|| anyhow::anyhow!("Invalid file path: '{}'", file.display()))?;

        if !bins.contains(&name.as_str()) {
            // binary not in the registry; skip it.
            continue;
        }

        let bin = paths::bin_dir().join(&name);
        disk::link_files(&file, &bin)?;
        map.insert(name, bin);
    }

    Ok(map)
}

pub fn link_npm(bins: Vec<&str>, pkg_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    link(&bins, &pkg_path.join("node_modules").join(".bin"))
}

// all package managers that use plain /bin
pub fn link_bin(bins: Vec<&str>, pkg_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    link(&bins, &pkg_path.join("bin"))
}

// all package managers that puts binaries at the root folder
pub fn link_root(bins: Vec<&str>, pkg_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    link(&bins, pkg_path)
}
