use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{disk, packages::install::link::asset::write_shim, paths};

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

// Gem packages need shims to isolate a Ruby environment there
pub fn link_gem(bins: Vec<&str>, pkg_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    let files = disk::list_files(&pkg_path.join("bin"))?;
    let mut linked = HashMap::new();

    let gem_home = pkg_path.to_string_lossy();
    let join = pkg_path.join("bin");

    for file in files {
        let name = file
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or_else(|| anyhow::anyhow!("Invalid file path: '{}'", file.display()))?;

        if !bins.contains(&name.as_str()) {
            // binary not in the registry; skip it.
            continue;
        }

        let shim = write_shim(
            &paths::bin_dir().join(&name),
            "ruby",
            &[],
            &join.join(&name),
            &[("GEM_HOME", &gem_home), ("GEM_PATH", &gem_home)],
        )?;

        linked.insert(name.to_owned(), shim);
    }

    Ok(linked)
}

pub fn link_pypi(bins: Vec<&str>, pkg_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    let files = disk::list_files(&pkg_path.join("bin"))?;
    let mut linked = HashMap::new();

    let dir = if cfg!(windows) { "Scripts" } else { "bin" };
    let join = pkg_path.join(dir);
    let interpreter = join.join("python3");

    for file in files {
        let name = file
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or_else(|| anyhow::anyhow!("Invalid file path: '{}'", file.display()))?;

        if !bins.contains(&name.as_str()) {
            // binary not in the registry; skip it.
            continue;
        }

        let shim = write_shim(
            &paths::bin_dir().join(&name),
            &interpreter.to_string_lossy(),
            &[],
            &join.join(&name),
            &[],
        )?;

        linked.insert(name.to_owned(), shim);
    }

    Ok(linked)
}
