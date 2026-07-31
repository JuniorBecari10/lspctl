use crate::{
    registry::model::{Asset, Build, Downloads, PackageManager, ResolvedEntry, SourceVariant},
    state::State,
};

pub fn install(
    entry: ResolvedEntry,
    state: &mut State,
    asset: Option<Asset>,
) -> anyhow::Result<()> {
    let res = match entry.source.variant {
        SourceVariant::PackageManager {
            manager,
            extra_packages,
        } => install_manager(manager, extra_packages),

        SourceVariant::Asset(_) => install_asset(asset),
        SourceVariant::Download(downloads) => install_download(downloads),
        SourceVariant::Build(builds) => install_build(builds),
    };

    // TODO: write to state BEFORE returning, and AFTER performing the work.
    // in other words, here.
    res
}

fn install_manager(manager: PackageManager, extra_packages: Vec<String>) -> anyhow::Result<()> {}

fn install_asset(asset: Option<Asset>) -> anyhow::Result<()> {
    todo!()
}

fn install_download(downloads: Downloads) -> anyhow::Result<()> {
    todo!()
}

fn install_build(builds: Vec<Build>) -> anyhow::Result<()> {
    todo!()
}
