use anyhow::Context;

use crate::{disk, packages::install::link::asset::write_shim, paths};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

fn link_with(
    bins: &[&str],
    bin_path: &Path,
    place: impl Fn(&str, &Path) -> anyhow::Result<PathBuf>,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    let files = disk::list_files(bin_path)?;
    let mut map = HashMap::new();

    for file in files {
        let name = file
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or_else(|| anyhow::anyhow!("Invalid file path: '{}'", file.display()))?;

        if !bins.contains(&name.as_str()) {
            continue; // binary not in the registry; skip it.
        }

        let linked = place(&name, &file)?;
        map.insert(name, linked);
    }

    Ok(map)
}

fn link(bins: &[&str], bin_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    link_with(bins, bin_path, |name, file| {
        let bin = paths::bin_dir().join(name);

        disk::link_files(file, &bin)?;
        Ok(bin)
    })
}

pub fn link_npm(bins: Vec<&str>, pkg_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    link(&bins, &pkg_path.join("node_modules").join(".bin"))
}

// all package managers that use plain /bin
pub fn link_bin(bins: Vec<&str>, pkg_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    link(&bins, &pkg_path.join("bin"))
}

// all package managers that put binaries at the root folder
pub fn link_root(bins: Vec<&str>, pkg_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    link(&bins, pkg_path)
}

// Gem packages need shims to isolate a Ruby environment
pub fn link_gem(bins: Vec<&str>, pkg_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    let bin_dir = pkg_path.join("bin");
    let gem_home = pkg_path.to_string_lossy().into_owned();

    link_with(&bins, &bin_dir, |name, file| {
        write_shim(
            &paths::bin_dir().join(name),
            "ruby",
            &[],
            file,
            &[("GEM_HOME", &gem_home), ("GEM_PATH", &gem_home)],
        )
    })
}

pub fn link_pypi(bins: Vec<&str>, pkg_path: &Path) -> anyhow::Result<HashMap<String, PathBuf>> {
    let scripts_dir = if cfg!(windows) { "Scripts" } else { "bin" };
    let bin_dir = pkg_path.join(scripts_dir);

    let python_bin = if cfg!(windows) {
        "python.exe"
    } else {
        "python3"
    };

    let interpreter = bin_dir.join(python_bin).to_string_lossy().into_owned();

    link_with(&bins, &bin_dir, |name, file| {
        write_shim(&paths::bin_dir().join(name), &interpreter, &[], file, &[])
    })
}

pub fn link_luarocks(
    bins: Vec<&str>,
    pkg_path: &Path,
    staging_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    let bin_dir = pkg_path.join("bin");

    rewrite_embedded_paths(&bin_dir, staging_path, pkg_path)?;
    link(&bins, &bin_dir)
}

fn rewrite_embedded_paths(dir: &Path, old_prefix: &Path, new_prefix: &Path) -> anyhow::Result<()> {
    let old = old_prefix.to_string_lossy();
    let new = new_prefix.to_string_lossy();

    for file in disk::list_files(dir)? {
        let contents = std::fs::read_to_string(&file)
            .with_context(|| format!("Failed to read '{}'", file.display()))?;

        if contents.contains(old.as_ref()) {
            let fixed = contents.replace(old.as_ref(), new.as_ref());
            std::fs::write(&file, fixed)
                .with_context(|| format!("Failed to rewrite '{}'", file.display()))?;
        }
    }

    Ok(())
}
