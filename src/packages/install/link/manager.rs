use anyhow::Context;

use crate::{
    disk,
    log::Format,
    packages::{install::link::asset::write_shim, link::util},
    paths,
    state::Executables,
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

fn link_with(
    bins: &[&str],
    bin_path: &Path,
    output: Option<&Path>,
    place: impl Fn(&str, &Path, &Path) -> anyhow::Result<PathBuf>, // (name, source, target_dir)
) -> anyhow::Result<HashMap<String, Executables>> {
    let files = disk::list_files(bin_path)?;
    let bin_dir = paths::bin_dir();
    let mut map = HashMap::new();

    for file in files {
        let name = file
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or_else(|| anyhow::anyhow!("Invalid file path: {}", file.display().quote()))?;

        if !bins.contains(&name.as_str()) {
            continue; // binary not in the registry; skip it.
        }

        let bin = place(&name, &file, &bin_dir)?;
        let out = output.map(|dir| place(&name, &file, dir)).transpose()?;

        map.insert(name, Executables(bin, out));
    }

    Ok(map)
}

fn link(
    bins: &[&str],
    bin_path: &Path,
    output: Option<&Path>,
) -> anyhow::Result<HashMap<String, Executables>> {
    link_with(bins, bin_path, output, |name, file, dir| {
        let target = dir.join(name);

        disk::link_files(file, &target)?;
        Ok(target)
    })
}

// ---

pub fn link_npm(
    bins: Vec<&str>,
    pkg_path: &Path,
    output: Option<&Path>,
) -> anyhow::Result<HashMap<String, Executables>> {
    link(&bins, &pkg_path.join("node_modules").join(".bin"), output)
}

// all package managers that use plain /bin
pub fn link_bin(
    bins: Vec<&str>,
    pkg_path: &Path,
    output: Option<&Path>,
) -> anyhow::Result<HashMap<String, Executables>> {
    link(&bins, &pkg_path.join("bin"), output)
}

// all package managers that put binaries at the root folder
pub fn link_root(
    bins: Vec<&str>,
    pkg_path: &Path,
    output: Option<&Path>,
) -> anyhow::Result<HashMap<String, Executables>> {
    link(&bins, pkg_path, output)
}

// Gem packages need shims to isolate a Ruby environment
pub fn link_gem(
    bins: Vec<&str>,
    pkg_path: &Path,
    output: Option<&Path>,
) -> anyhow::Result<HashMap<String, Executables>> {
    let bin_dir = pkg_path.join("bin");
    let gem_home = pkg_path.to_string_lossy().into_owned();

    link_with(&bins, &bin_dir, output, |name, file, dir| {
        write_shim(
            &dir.join(name),
            "ruby",
            &[],
            file,
            &[("GEM_HOME", &gem_home), ("GEM_PATH", &gem_home)],
        )
    })
}

pub fn link_pypi(
    bins: Vec<&str>,
    pkg_path: &Path,
    output: Option<&Path>,
) -> anyhow::Result<HashMap<String, Executables>> {
    let scripts_dir = if cfg!(windows) { "Scripts" } else { "bin" };
    let bin_dir = pkg_path.join(scripts_dir);

    let python_bin = if cfg!(windows) {
        "python.exe"
    } else {
        "python3"
    };

    let interpreter = bin_dir.join(python_bin).to_string_lossy().into_owned();

    link_with(&bins, &bin_dir, output, |name, file, dir| {
        write_shim(&dir.join(name), &interpreter, &[], file, &[])
    })
}

pub fn link_luarocks(
    bins: Vec<&str>,
    pkg_path: &Path,
    staging_path: &Path,
    output: Option<&Path>,
) -> anyhow::Result<HashMap<String, Executables>> {
    let bin_dir = pkg_path.join("bin");

    util::rewrite_embedded_paths(&bin_dir, staging_path, pkg_path)?;
    link(&bins, &bin_dir, output)
}

pub fn link_composer(
    bins: Vec<&str>,
    pkg_path: &Path,
    output: Option<&Path>,
) -> anyhow::Result<HashMap<String, Executables>> {
    link(&bins, &pkg_path.join("vendor").join("bin"), output)
}
