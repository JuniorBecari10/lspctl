#![allow(unused)]

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    packages::install::link::manager,
    paths,
    registry::model::{Build, PackageManager, ResolvedDownloads, ResolvedEntry},
};

pub fn link_manager(
    entry: &ResolvedEntry,
    manager: PackageManager,
    pkg_path: &Path,
    tmp_pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    let hash_map = entry.bin.clone().unwrap_or_default();
    let bins = hash_map.keys().map(String::as_str).collect();

    match manager {
        PackageManager::Npm => manager::link_npm(bins, pkg_path),
        PackageManager::PyPI => manager::link_pypi(bins, pkg_path),
        PackageManager::Go => manager::link_bin(bins, pkg_path),
        PackageManager::Cargo => manager::link_bin(bins, pkg_path),
        PackageManager::Gem => manager::link_gem(bins, pkg_path),
        PackageManager::Composer => anyhow::bail!("todo"),
        PackageManager::LuaRocks => manager::link_luarocks(bins, pkg_path, tmp_pkg_path),
        PackageManager::Opam => anyhow::bail!("todo"),
        PackageManager::NuGet => manager::link_root(bins, pkg_path),
    }
}

pub fn link_asset(
    entry: &ResolvedEntry,
    pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    let mut map = HashMap::new();

    for (name, value) in entry.bin.iter().flatten() {
        let bin = paths::bin_dir().join(name);
        let target = super::asset::get_target(name, value, &bin, pkg_path)?;

        map.insert(name.clone(), target);
    }

    Ok(map)
}

pub fn link_download(
    entry: &ResolvedEntry,
    downloads: &ResolvedDownloads,
    pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    anyhow::bail!("todo")
}

pub fn link_build(
    entry: &ResolvedEntry,
    build: &Build,
    pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    anyhow::bail!("todo")
}
