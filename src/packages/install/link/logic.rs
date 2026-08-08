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
    tmp_pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    match manager {
        PackageManager::Npm => manager::link_npm(
            entry.bin.clone().unwrap_or_default().keys().collect(),
            tmp_pkg_path,
        ),
        PackageManager::PyPI => todo!(),
        PackageManager::Go => todo!(),
        PackageManager::Cargo => todo!(),
        PackageManager::Gem => todo!(),
        PackageManager::Composer => todo!(),
        PackageManager::LuaRocks => todo!(),
        PackageManager::Opam => todo!(),
        PackageManager::NuGet => todo!(),
    }
}

pub fn link_asset(
    entry: &ResolvedEntry,
    asset: &Asset,
    tmp_pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    todo!()
}

pub fn link_download(
    entry: &ResolvedEntry,
    downloads: &ResolvedDownloads,
    tmp_pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    todo!()
}

pub fn link_build(
    entry: &ResolvedEntry,
    build: &Build,
    tmp_pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    todo!()
}
