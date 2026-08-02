use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    packages::{shim, util},
    paths,
    registry::model::{
        Asset, Build, PackageManager, ResolvedDownloads, ResolvedEntry, ResolvedVariant,
    },
    state::State,
};

pub fn install(entry: ResolvedEntry, state: &mut State) -> anyhow::Result<()> {
    let tmp_pkg_path = paths::tmp_dir()
        .join(&entry.name)
        .join(&entry.source.purl.version);

    fs::create_dir_all(&tmp_pkg_path)?;

    // install in tmp and move it to the definitive folder
    install_by_variant(&entry, &tmp_pkg_path)?;
    util::move_package(&entry.name)?;

    // make shims/symlinks in bin and add the entry to state
    let bins = make_shims(&entry, &tmp_pkg_path)?;
    state.add_entry(&entry, bins);

    Ok(())
}

fn install_by_variant(entry: &ResolvedEntry, tmp_pkg_path: &Path) -> anyhow::Result<()> {
    match &entry.source.variant {
        ResolvedVariant::PackageManager {
            manager,
            extra_packages,
        } => install_manager(&entry, *manager, extra_packages, tmp_pkg_path),

        ResolvedVariant::Asset(asset) => install_asset(&entry, asset, tmp_pkg_path),
        ResolvedVariant::Download(downloads) => install_download(&entry, downloads, tmp_pkg_path),
        ResolvedVariant::Build(build) => install_build(&entry, build, tmp_pkg_path),
    }
}

fn make_shims(
    entry: &ResolvedEntry,
    tmp_pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    match &entry.source.variant {
        ResolvedVariant::PackageManager {
            manager,
            extra_packages: _,
        } => shim::shim_manager(&entry, *manager, &tmp_pkg_path),

        ResolvedVariant::Asset(asset) => shim::shim_asset(&entry, asset, tmp_pkg_path),

        ResolvedVariant::Download(downloads) => {
            shim::shim_download(&entry, downloads, tmp_pkg_path)
        }

        ResolvedVariant::Build(build) => shim::shim_build(&entry, build, tmp_pkg_path),
    }
}

// ---

// the job of these functions is to perform the work to
// make the package sit in the tmp folder in the correct folder hierarchy: name / version / data.
// the rest (make shims, move to definitive folder and update state) is handled by the functions above.

fn install_manager(
    entry: &ResolvedEntry,
    manager: PackageManager,
    extra_packages: &[String],
    tmp_pkg_path: &Path,
) -> anyhow::Result<()> {
    let command = util::get_install_command(
        manager,
        &entry.source.purl.qualified_package_name(),
        &entry.source.purl.version,
        extra_packages,
        tmp_pkg_path,
    );

    util::run_command(command, tmp_pkg_path)
}

fn install_asset(entry: &ResolvedEntry, asset: &Asset, tmp_pkg_path: &Path) -> anyhow::Result<()> {
    todo!()
}

fn install_download(
    entry: &ResolvedEntry,
    downloads: &ResolvedDownloads,
    tmp_pkg_path: &Path,
) -> anyhow::Result<()> {
    todo!()
}

fn install_build(entry: &ResolvedEntry, build: &Build, tmp_pkg_path: &Path) -> anyhow::Result<()> {
    todo!()
}
