mod ast;
mod eval;
mod parser;
pub mod public;
mod segment;
mod token;

use crate::registry::model::{
    Asset, Build, Download, Downloads, Entry, Registry, Source, SourceVariant, VersionOverride,
};

pub fn resolve_templates(reg: Registry) -> anyhow::Result<Registry> {
    Ok(Registry(
        reg.0
            .into_iter()
            .map(resolve_entry)
            .collect::<anyhow::Result<_>>()?,
    ))
}

fn resolve_entry(e: Entry) -> anyhow::Result<Entry> {
    let source = resolve_source(e.source)?;

    Ok(Entry {
        bin: e
            .bin
            .map(|b| public::parse_template_hashmap(b, &source))
            .transpose()?,

        source,
        ..e
    })
}

fn resolve_source(s: Source) -> anyhow::Result<Source> {
    let snapshot = s.clone();

    Ok(Source {
        variant: s
            .variant
            .map(|v| resolve_variant(v, &snapshot))
            .transpose()?,

        version_overrides: s
            .version_overrides
            .map(|vos| {
                vos.into_iter()
                    .map(|vo| resolve_version_override(vo, &snapshot))
                    .collect::<anyhow::Result<_>>()
            })
            .transpose()?,

        bin: s
            .bin
            .map(|b| public::parse_template(b, &snapshot))
            .transpose()?,

        ..s
    })
}

fn resolve_variant(v: SourceVariant, source: &Source) -> anyhow::Result<SourceVariant> {
    match v {
        SourceVariant::PackageManager {
            manager,
            extra_packages,
        } => Ok(SourceVariant::PackageManager {
            manager,
            extra_packages,
        }),

        SourceVariant::Asset(assets) => Ok(SourceVariant::Asset(
            assets
                .into_iter()
                .map(|a| resolve_asset(a, source))
                .collect::<anyhow::Result<_>>()?,
        )),

        SourceVariant::Download(downloads) => Ok(SourceVariant::Download(resolve_downloads(
            downloads, source,
        )?)),

        SourceVariant::Build(builds) => Ok(SourceVariant::Build(
            builds
                .into_iter()
                .map(|b| resolve_build(b, source))
                .collect::<anyhow::Result<_>>()?,
        )),
    }
}

fn resolve_version_override(
    vo: VersionOverride,
    source: &Source,
) -> anyhow::Result<VersionOverride> {
    Ok(VersionOverride {
        variant: resolve_variant(vo.variant, source)?,
        ..vo
    })
}

fn resolve_asset(a: Asset, source: &Source) -> anyhow::Result<Asset> {
    Ok(Asset {
        files: a
            .files
            .into_iter()
            .map(|f| public::parse_template(f, source))
            .collect::<anyhow::Result<_>>()?,

        bin: a
            .bin
            .map(|bin| bin.try_map(|b| public::parse_template(b, source)))
            .transpose()?,

        variables: a
            .variables
            .into_iter()
            .map(|(k, v)| {
                Ok((
                    public::parse_template(k, source)?,
                    v.try_map(|vars| public::parse_template(vars, source))?,
                ))
            })
            .collect::<anyhow::Result<_>>()?,

        ..a
    })
}

fn resolve_downloads(d: Downloads, source: &Source) -> anyhow::Result<Downloads> {
    match d {
        Downloads::Simple { file } => Ok(Downloads::Simple {
            file: public::parse_template(file, source)?,
        }),

        Downloads::Detailed(downloads) => Ok(Downloads::Detailed(
            downloads
                .into_iter()
                .map(|d| resolve_download(d, source))
                .collect::<anyhow::Result<_>>()?,
        )),
    }
}

fn resolve_download(d: Download, source: &Source) -> anyhow::Result<Download> {
    Ok(Download {
        files: public::parse_template_hashmap(d.files, source)?,
        bin: d
            .bin
            .map(|b| public::parse_template(b, source))
            .transpose()?,

        ..d
    })
}

fn resolve_build(b: Build, source: &Source) -> anyhow::Result<Build> {
    Ok(Build {
        bin: b
            .bin
            .map(|bin| bin.try_map(|b| public::parse_template(b, source)))
            .transpose()?,

        env: b
            .env
            .map(|e| public::parse_template_hashmap(e, source))
            .transpose()?,

        extra: public::parse_template_hashmap(b.extra, source)?,
        ..b
    })
}
