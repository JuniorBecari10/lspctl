use std::{collections::HashMap, fs, path::Path};

use crate::{
    packages::util,
    paths,
    registry::model::{
        Asset, Build, Downloads, Entry, PackageManager, ResolvedEntry, SourceVariant,
    },
    state::State,
};

pub fn install(
    entry: ResolvedEntry,
    state: &mut State,
    asset: Option<Asset>,
) -> anyhow::Result<()> {
    let tmp_pkg_path = paths::tmp_dir()
        .join(&entry.name)
        .join(&entry.source.purl.version);

    fs::create_dir_all(&tmp_pkg_path)?;

    match &entry.source.variant {
        SourceVariant::PackageManager {
            manager,
            extra_packages,
        } => install_manager(&entry, *manager, extra_packages, &tmp_pkg_path),

        SourceVariant::Asset(_) => install_asset(&entry, asset),
        SourceVariant::Download(downloads) => install_download(&entry, downloads),
        SourceVariant::Build(builds) => install_build(&entry, builds),
    }?;

    // resolve bins and shims
    util::move_package(&entry.name)?;
    state.add_entry(&entry, HashMap::new());
    Ok(())
}

// ---

// the job of these functions is to perform the work to
// make the package sit in the tmp folder in the correct folder hierarchy: name / version / data.
// the rest (make shims, move to definitive folder and update state) is handled by the function above.

// TODO: return the bins here, since they are in different places depending on the install method?
// when installing, make all files be owned by the user

fn install_manager(
    entry: &Entry,
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

fn install_asset(entry: &Entry, asset: Option<Asset>) -> anyhow::Result<()> {
    todo!()
}

fn install_download(entry: &Entry, downloads: &Downloads) -> anyhow::Result<()> {
    todo!()
}

fn install_build(entry: &Entry, builds: &[Build]) -> anyhow::Result<()> {
    todo!()
}
