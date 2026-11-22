#![allow(unused)]

pub mod asset;
mod manager;
mod util;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    paths,
    registry::model::{Build, PackageManager, ResolvedDownloads, ResolvedEntry},
    state::Executables,
};

pub fn link_manager(
    entry: &ResolvedEntry,
    manager: PackageManager,
    pkg_path: &Path,
    tmp_pkg_path: &Path,
    output: Option<&Path>,
) -> anyhow::Result<HashMap<String, Executables>> {
    let hash_map = entry.bin.clone().unwrap_or_default();
    let bins = hash_map.keys().map(String::as_str).collect();

    match manager {
        PackageManager::Npm => manager::link_npm(bins, pkg_path, output),
        PackageManager::PyPI => manager::link_pypi(bins, pkg_path, output),

        PackageManager::Go | PackageManager::Cargo | PackageManager::Opam => {
            manager::link_bin(bins, pkg_path, output)
        }

        PackageManager::Gem => manager::link_gem(bins, pkg_path, output),
        PackageManager::Composer => manager::link_composer(bins, pkg_path, output),
        PackageManager::LuaRocks => manager::link_luarocks(bins, pkg_path, tmp_pkg_path, output),
        PackageManager::NuGet => manager::link_root(bins, pkg_path, output),
    }
}

pub fn link_asset_and_download(
    entry: &ResolvedEntry,
    pkg_path: &Path,
    output: Option<&Path>,
) -> anyhow::Result<HashMap<String, Executables>> {
    let mut map = HashMap::new();

    for (name, value) in entry.bin.iter().flatten() {
        let get_target = |dir: &Path| {
            let bin = dir.join(name);
            asset::get_target(name, value, &bin, pkg_path)
        };

        let bin = get_target(&paths::bin_dir())?;
        let out = output.map(get_target).transpose()?;

        map.insert(name.clone(), Executables(bin, out));
    }

    Ok(map)
}

pub fn link_build(
    entry: &ResolvedEntry,
    build: &Build,
    pkg_path: &Path,
    output: Option<&Path>,
) -> anyhow::Result<HashMap<String, Executables>> {
    anyhow::bail!("todo")
}
