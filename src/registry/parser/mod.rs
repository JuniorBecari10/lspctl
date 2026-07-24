mod platform;
mod template;

use std::str::FromStr;

use packageurl::PackageUrl;

use crate::registry::model::{
    Asset, Build, Download, Downloads, Entry, InstallKind, Purl, RawAsset, RawBuild, RawDownload,
    RawDownloads, RawEntry, RawRegistry, RawSource, RawSourceVariant, RawVersionOverride, Registry,
    Source, SourceVariant, VersionOverride,
};

pub fn parse_registry(raw: RawRegistry) -> anyhow::Result<Registry> {
    Ok(template::resolve_templates(Registry(
        raw.0
            .into_iter()
            .map(parse_entry)
            .collect::<anyhow::Result<_>>()?,
    ))?)
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
    let purl: Purl = PackageUrl::from_str(&raw.id)?.try_into()?;

    Ok(Source {
        variant: raw
            .variant
            .map(|v| parse_variant(v, purl.kind))
            .transpose()?,
        supported_platforms: platform::convert_platforms(raw.supported_platforms)?,
        version_overrides: raw
            .version_overrides
            .map(|raw_vo| {
                raw_vo
                    .into_iter()
                    .map(|vo| parse_version_override(vo, purl.kind))
                    .collect::<anyhow::Result<_>>()
            })
            .transpose()?,
        purl, // used after because of the borrow checker
        bin: raw.bin,
    })
}

fn parse_variant(raw: RawSourceVariant, kind: InstallKind) -> anyhow::Result<SourceVariant> {
    match raw {
        RawSourceVariant::Asset { asset } => Ok(SourceVariant::Asset(
            Into::<Vec<_>>::into(asset)
                .into_iter()
                .map(parse_asset)
                .collect::<anyhow::Result<_>>()?,
        )),

        RawSourceVariant::Download { download } => {
            Ok(SourceVariant::Download(parse_downloads(download)?))
        }

        RawSourceVariant::Build { build } => Ok(SourceVariant::Build(
            Into::<Vec<_>>::into(build)
                .into_iter()
                .map(parse_build)
                .collect::<anyhow::Result<_>>()?,
        )),

        RawSourceVariant::ExtraPackages { extra_packages } => Ok(SourceVariant::PackageManager {
            manager: kind.try_into()?,
            extra_packages,
        }),
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
        supported_platforms: platform::convert_platforms(raw.supported_platforms)?,
    })
}

fn parse_asset(raw: RawAsset) -> anyhow::Result<Asset> {
    Ok(Asset {
        targets: platform::convert_platforms(raw.target.map(Into::into))?,
        files: Into::<Vec<_>>::into(raw.file)
            .into_iter()
            .map(template::parse_template)
            .collect(),
        bin: raw.bin.map(|m| m.map(template::parse_template)),
        variables: raw
            .variables
            .into_iter()
            .map(|(k, v)| (template::parse_template(k), v.map(template::parse_template)))
            .collect(),
    })
}

fn parse_downloads(raw: RawDownloads) -> anyhow::Result<Downloads> {
    match raw {
        RawDownloads::Simple { file } => Ok(Downloads::Simple {
            file: template::parse_template(file),
        }),

        RawDownloads::Detailed(downs) => Ok(Downloads::Detailed(
            Into::<Vec<_>>::into(downs)
                .into_iter()
                .map(parse_download)
                .collect::<anyhow::Result<_>>()?,
        )),
    }
}

fn parse_download(raw: RawDownload) -> anyhow::Result<Download> {
    Ok(Download {
        targets: platform::convert_platforms(raw.target.map(Into::into))?,
        files: template::parse_template_hashmap(raw.files),
        bin: raw.bin.map(template::parse_template),
    })
}

fn parse_build(raw: RawBuild) -> anyhow::Result<Build> {
    Ok(Build {
        command: raw.run,
        targets: platform::convert_platforms(raw.target.map(Into::into))?,
        bin: raw.bin.map(|bin| bin.map(template::parse_template)),
        env: raw.env.map(template::parse_template_hashmap),
        staged: raw.staged,
        extra: template::parse_template_hashmap(raw.extra),
    })
}
