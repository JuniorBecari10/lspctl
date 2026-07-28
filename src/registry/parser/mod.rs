mod platform;
pub mod template;

use std::str::FromStr;

use packageurl::PackageUrl;

use crate::registry::model::*;

pub fn parse_registry(raw: RawRegistry) -> anyhow::Result<Registry> {
    Ok(Registry(
        raw.0
            .into_iter()
            .map(parse_entry)
            .collect::<anyhow::Result<_>>()?,
    ))
}

fn parse_entry(raw: RawEntry) -> anyhow::Result<Entry> {
    Ok(Entry {
        name: raw.name,
        description: raw.description.trim().into(),
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
    let purl: Purl = PackageUrl::from_str(&raw.id)?.try_into()?;

    Ok(Source {
        variant: parse_variant(raw.variant, purl.kind)?,

        version_overrides: raw
            .version_overrides
            .map(|vos| {
                vos.into_iter()
                    .map(|vo| parse_version_override(vo, purl.kind))
                    .collect::<anyhow::Result<_>>()
            })
            .transpose()?,

        supported_platforms: platform::convert_platforms(raw.supported_platforms)?,

        purl, // used after because of the borrow checker
        bin: raw.bin,
    })
}

fn parse_variant(
    raw: Option<RawSourceVariant>,
    kind: InstallKind,
) -> anyhow::Result<SourceVariant> {
    let manager: Result<PackageManager, _> = kind.try_into();

    match raw {
        Some(RawSourceVariant::ExtraPackages { extra_packages }) => {
            Ok(SourceVariant::PackageManager {
                manager: manager?,
                extra_packages,
            })
        }

        Some(RawSourceVariant::Asset { asset }) => Ok(SourceVariant::Asset(
            Into::<Vec<_>>::into(asset)
                .into_iter()
                .map(parse_asset)
                .collect::<anyhow::Result<_>>()?,
        )),

        Some(RawSourceVariant::Download { download }) => {
            Ok(SourceVariant::Download(parse_downloads(download)?))
        }

        Some(RawSourceVariant::Build { build }) => Ok(SourceVariant::Build(
            Into::<Vec<_>>::into(build)
                .into_iter()
                .map(parse_build)
                .collect::<anyhow::Result<_>>()?,
        )),

        // edge case where 'extra_packages' is not present but 'kind' is a package manager
        None => manager
            .map(|m| SourceVariant::PackageManager { manager: m, extra_packages: vec![] })
            .map_err(|_| anyhow::anyhow!(
                "Package has no source variant and purl kind '{kind}' has no known package manager"
            )
        ),
    }
}

fn parse_version_override(
    raw: RawVersionOverride,
    kind: InstallKind,
) -> anyhow::Result<VersionOverride> {
    Ok(VersionOverride {
        constraint: raw.constraint,
        id: raw.id,
        variant: parse_variant(Some(raw.variant), kind)?,
        supported_platforms: platform::convert_platforms(raw.supported_platforms)?,
    })
}

fn parse_asset(raw: RawAsset) -> anyhow::Result<Asset> {
    Ok(Asset {
        targets: platform::convert_platforms(raw.target.map(Into::into))?,
        files: raw.file.into(),
        bin: raw.bin,
        variables: raw.variables,
    })
}

fn parse_downloads(raw: RawDownloads) -> anyhow::Result<Downloads> {
    match raw {
        RawDownloads::Simple { file } => Ok(Downloads::Simple { file }),

        RawDownloads::Detailed(downloads) => Ok(Downloads::Detailed(
            Into::<Vec<_>>::into(downloads)
                .into_iter()
                .map(parse_download)
                .collect::<anyhow::Result<_>>()?,
        )),
    }
}

fn parse_download(raw: RawDownload) -> anyhow::Result<Download> {
    Ok(Download {
        targets: platform::convert_platforms(raw.target.map(Into::into))?,
        files: raw.files,
        bin: raw.bin,
    })
}

fn parse_build(raw: RawBuild) -> anyhow::Result<Build> {
    Ok(Build {
        command: raw.run,
        targets: platform::convert_platforms(raw.target.map(Into::into))?,
        bin: raw.bin,
        env: raw.env,
        staged: raw.staged,
        extra: raw.extra,
    })
}
