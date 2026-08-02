use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::registry::model::{Asset, Build, PackageManager, ResolvedDownloads, ResolvedEntry};

pub fn shim_manager(
    entry: &ResolvedEntry,
    manager: PackageManager,
    tmp_pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    todo!()
}

pub fn shim_asset(
    entry: &ResolvedEntry,
    asset: &Asset,
    tmp_pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    todo!()
}

pub fn shim_download(
    entry: &ResolvedEntry,
    downloads: &ResolvedDownloads,
    tmp_pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    todo!()
}

pub fn shim_build(
    entry: &ResolvedEntry,
    build: &Build,
    tmp_pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    todo!()
}
