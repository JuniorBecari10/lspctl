mod util;

use packageurl::PackageUrl;

use crate::registry::{
    model::{
        Asset, Build, Download, Downloads, Entry, InstallKind, Purl, RawAsset, RawBuild,
        RawDownload, RawDownloads, RawEntry, RawRegistry, RawSource, RawSourceVariant,
        RawVersionOverride, Registry, Source, SourceVariant, VersionOverride,
    },
    parser::util::convert_platforms,
};

pub fn parse_registry(raw: RawRegistry) -> anyhow::Result<Registry> {
    Ok(Registry(
        raw.0
            .into_iter()
            .map(parse_entry)
            .collect::<anyhow::Result<Vec<_>>>()?,
    ))
}

fn parse_entry(raw: RawEntry) -> anyhow::Result<Entry> {
    Ok(Entry {
        name: raw.name,
        description: raw.description,
        homepage: raw.homepage,
        licenses: raw.licenses,
        languages: raw.languages,
        categories: raw.categories,
        source: parse_source(raw.source)?,
        bin: raw.bin,
        deprecation: raw.deprecation,
    })
}

fn parse_source(raw: RawSource) -> anyhow::Result<Source> {
    let purl: Purl = PackageUrl::new(raw.id, "purl")?.try_into()?;

    Ok(Source {
        variant: parse_variant(raw.variant, purl.kind)?,
        supported_platforms: convert_platforms(raw.supported_platforms)?,
        version_overrides: raw
            .version_overrides
            .map(|os| {
                os.into_iter()
                    .map(|o| parse_version_override(o, purl.kind))
                    .collect::<anyhow::Result<Vec<_>>>()
            })
            .transpose()?,
        purl, // used after because of the borrow checker
        bin: raw.bin,
    })
}

fn parse_variant(raw: RawSourceVariant, kind: InstallKind) -> anyhow::Result<SourceVariant> {
    match raw {
        RawSourceVariant {
            extra_packages: Some(pkgs),
            ..
        } if kind.is_package_manager() => Ok(SourceVariant::PackageManager {
            manager: kind.try_into()?,
            extra_packages: pkgs,
        }),

        RawSourceVariant {
            assets: Some(assets),
            ..
        } => Ok(SourceVariant::Asset(
            Into::<Vec<_>>::into(assets)
                .into_iter()
                .map(parse_asset)
                .collect::<anyhow::Result<Vec<_>>>()?,
        )),

        RawSourceVariant {
            download: Some(downloads),
            ..
        } => Ok(SourceVariant::Download(parse_downloads(downloads)?)),

        RawSourceVariant {
            build: Some(build), ..
        } => Ok(SourceVariant::Build(
            Into::<Vec<_>>::into(build)
                .into_iter()
                .map(parse_build)
                .collect::<anyhow::Result<Vec<_>>>()?,
        )),

        _ => anyhow::bail!("invalid variant {raw:?} and/or kind {kind:?}"), // TODO: pretty-print
    }
}

fn parse_version_override(
    raw: RawVersionOverride,
    kind: InstallKind,
) -> anyhow::Result<VersionOverride> {
    Ok(VersionOverride {
        constraint: raw.constraint,
        id: raw.id,
        variant: parse_variant(raw.variant, kind)?,
        supported_platforms: convert_platforms(raw.supported_platforms)?,
    })
}

fn parse_asset(raw: RawAsset) -> anyhow::Result<Asset> {
    Ok(Asset {
        targets: convert_platforms(raw.target.map(|t| t.into()))?,
        files: raw.file.into(),
        bin: raw.bin,
        extra: raw.extra,
    })
}

fn parse_downloads(raw: RawDownloads) -> anyhow::Result<Downloads> {
    match raw {
        RawDownloads::Simple { file } => Ok(Downloads::Simple { file }),
        RawDownloads::Detailed(downs) => Ok(Downloads::Detailed(
            Into::<Vec<_>>::into(downs)
                .into_iter()
                .map(parse_download)
                .collect::<anyhow::Result<Vec<_>>>()?,
        )),
    }
}

fn parse_download(raw: RawDownload) -> anyhow::Result<Download> {
    Ok(Download {
        targets: convert_platforms(raw.target.map(|t| t.into()))?,
        files: raw.files,
        bin: raw.bin,
    })
}

fn parse_build(raw: RawBuild) -> anyhow::Result<Build> {
    Ok(Build {
        command: raw.run,
        targets: convert_platforms(raw.target.map(|t| t.into()))?,
        bin: raw.bin,
        env: raw.env,
        staged: raw.staged,
        extra: raw.extra,
    })
}
