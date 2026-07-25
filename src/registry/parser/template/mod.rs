pub mod parser;
mod token;

pub use parser::*;

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
            .map(|b| parser::parse_template_hashmap(b, &source))
            .transpose()?,
        source,
        ..e
    })
}

// it's your fault, js-debug-adapter!
fn resolve_source(s: Source) -> anyhow::Result<Source> {
    let cloned = s.clone(); // this bin should use the older version of s

    Ok(Source {
        variant: s.variant.map(|v| resolve_variant(v, &cloned)).transpose()?,
        version_overrides: s
            .version_overrides
            .map(|vo| {
                vo.into_iter()
                    .map(|vo| resolve_version_override(vo, &cloned))
                    .collect::<anyhow::Result<_>>()
            })
            .transpose()?,
        bin: s
            .bin
            .map(|b| parser::parse_template(b, &cloned))
            .transpose()?,
        ..s
    })
}

fn resolve_variant(v: SourceVariant, s: &Source) -> anyhow::Result<SourceVariant> {
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
                .map(|a| resolve_asset(a, s))
                .collect::<anyhow::Result<_>>()?,
        )),

        SourceVariant::Download(downloads) => {
            Ok(SourceVariant::Download(resolve_downloads(downloads, s)?))
        }

        SourceVariant::Build(builds) => Ok(SourceVariant::Build(
            builds
                .into_iter()
                .map(|b| resolve_build(b, s))
                .collect::<anyhow::Result<_>>()?,
        )),
    }
}

fn resolve_version_override(vo: VersionOverride, s: &Source) -> anyhow::Result<VersionOverride> {
    Ok(VersionOverride {
        variant: resolve_variant(vo.variant, s)?,
        ..vo
    })
}

fn resolve_asset(a: Asset, s: &Source) -> anyhow::Result<Asset> {
    Ok(Asset {
        files: a
            .files
            .into_iter()
            .map(|f| parser::parse_template(f, s))
            .collect::<anyhow::Result<_>>()?,

        bin: a
            .bin
            .map(|m| m.try_map(|st| parser::parse_template(st, s)))
            .transpose()?,

        variables: a
            .variables
            .into_iter()
            .map(|(k, v)| {
                Ok((
                    parser::parse_template(k, s)?,
                    v.try_map(|vars| parser::parse_template(vars, s))?,
                ))
            })
            .collect::<anyhow::Result<_>>()?,
        ..a
    })
}

fn resolve_downloads(d: Downloads, s: &Source) -> anyhow::Result<Downloads> {
    match d {
        Downloads::Simple { file } => Ok(Downloads::Simple {
            file: parser::parse_template(file, s)?,
        }),

        Downloads::Detailed(downloads) => Ok(Downloads::Detailed(
            downloads
                .into_iter()
                .map(|d| resolve_download(d, s))
                .collect::<anyhow::Result<_>>()?,
        )),
    }
}

fn resolve_download(d: Download, s: &Source) -> anyhow::Result<Download> {
    Ok(Download {
        files: parser::parse_template_hashmap(d.files, s)?,
        bin: d.bin.map(|b| parser::parse_template(b, s)).transpose()?,
        ..d
    })
}

fn resolve_build(b: Build, s: &Source) -> anyhow::Result<Build> {
    Ok(Build {
        bin: b
            .bin
            .map(|bb| bb.try_map(|st| parser::parse_template(st, s)))
            .transpose()?,

        env: b
            .env
            .map(|e| parser::parse_template_hashmap(e, s))
            .transpose()?,

        extra: parser::parse_template_hashmap(b.extra, s)?,
        ..b
    })
}
