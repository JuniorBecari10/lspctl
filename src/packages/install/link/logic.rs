#![allow(unused)]

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    packages::install::link::manager,
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
        PackageManager::PyPI => todo!(),
        PackageManager::Go => manager::link_go,
        PackageManager::Cargo => todo!(),
        PackageManager::Gem => todo!(),
        PackageManager::Composer => todo!(),
        PackageManager::LuaRocks => todo!(),
        PackageManager::Opam => todo!(),
        PackageManager::NuGet => todo!(),
    };

    link_fn(bins, pkg_path)
}

pub fn link_asset(
    entry: &ResolvedEntry,
    asset: &Asset,
    pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    todo!()
}

pub fn link_download(
    entry: &ResolvedEntry,
    downloads: &ResolvedDownloads,
    pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    todo!()
}

pub fn link_build(
    entry: &ResolvedEntry,
    build: &Build,
    pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    todo!()
}
