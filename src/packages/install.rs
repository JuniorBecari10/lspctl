use crate::{
    packages::util,
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
    let res = match &entry.source.variant {
        SourceVariant::PackageManager {
            manager,
            extra_packages,
        } => install_manager(&entry, *manager, extra_packages),

        SourceVariant::Asset(_) => install_asset(&entry, asset),
        SourceVariant::Download(downloads) => install_download(&entry, &downloads),
        SourceVariant::Build(builds) => install_build(&entry, &builds),
    };

    // TODO: write to state BEFORE returning, and AFTER performing the work.
    // in other words, here.
    res
}

fn install_manager(
    entry: &Entry,
    manager: PackageManager,
    extra_packages: &[String],
) -> anyhow::Result<()> {
    let command = util::get_install_command(manager, &entry.name, &entry.source.purl.version);
    // TODO: run command
    todo!()
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
