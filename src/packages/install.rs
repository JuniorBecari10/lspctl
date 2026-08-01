use std::{collections::HashMap, fs::create_dir_all, path::Path};

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
    let tmp_path = paths::tmp_dir()
        .join(&entry.name)
        .join(&entry.source.purl.version);

    create_dir_all(&tmp_path)?;

    match &entry.source.variant {
        SourceVariant::PackageManager {
            manager,
            extra_packages,
        } => install_manager(&entry, *manager, extra_packages, &tmp_path),

        SourceVariant::Asset(_) => install_asset(&entry, asset),
        SourceVariant::Download(downloads) => install_download(&entry, downloads),
        SourceVariant::Build(builds) => install_build(&entry, builds),
    }?;

    // resolve bins, shims and move to definitive location
    state.add_entry(&entry, HashMap::new());
    Ok(())
}

// ---

// the job of these functions is to perform the work to
// make the package sit in the tmp folder in the correct folder hierarchy: name / version / data.
// the rest (make shims, move to definitive folder and update state) is handled by the function above.

fn install_manager(
    entry: &Entry,
    manager: PackageManager,
    extra_packages: &[String],
    tmp_path: &Path,
) -> anyhow::Result<()> {
    let command = util::get_install_command(
        manager,
        &entry.name,
        &entry.source.purl.version,
        extra_packages,
        tmp_path,
    );

    util::run_command(command, tmp_path)
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
