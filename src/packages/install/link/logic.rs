#![allow(unused)]

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    packages::install::link::manager,
    paths,
    registry::model::{Asset, Build, PackageManager, ResolvedDownloads, ResolvedEntry},
};

pub fn link_manager(
    entry: &ResolvedEntry,
    manager: PackageManager,
    pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    let hash_map = entry.bin.clone().unwrap_or_default();
    let bins = hash_map.keys().map(String::as_str).collect();

    let link_fn = match manager {
        PackageManager::Npm => manager::link_npm,
        PackageManager::PyPI => anyhow::bail!("todo"),
        PackageManager::Go => manager::link_go,
        PackageManager::Cargo => anyhow::bail!("todo"),
        PackageManager::Gem => anyhow::bail!("todo"),
        PackageManager::Composer => anyhow::bail!("todo"),
        PackageManager::LuaRocks => anyhow::bail!("todo"),
        PackageManager::Opam => anyhow::bail!("todo"),
        PackageManager::NuGet => anyhow::bail!("todo"),
    };

    link_fn(bins, pkg_path)
}

pub fn link_asset(
    entry: &ResolvedEntry,
    pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    let mut map = HashMap::new();

    for (name, value) in entry.bin.iter().flatten() {
        let bin = paths::bin_dir().join(name);
        let target = super::asset::get_target(value, pkg_path)?;

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
