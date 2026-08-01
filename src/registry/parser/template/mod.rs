mod ast;
pub mod context;
mod eval;
mod parser;
pub mod public;
mod segment;
mod select;
mod token;

use crate::registry::{
    model::{Asset, Build, Download, Downloads, Entry, Source, SourceVariant, VersionOverride},
    parser::template::context::ResolveContext,
};

pub fn resolve_entry(e: Entry, ctx: &ResolveContext) -> anyhow::Result<Entry> {
    let source = resolve_source(e.source, ctx)?;

    Ok(Entry {
        bin: e
            .bin
            .map(|b| public::parse_template_hashmap(b, ctx))
            .transpose()?,

        source,
        ..e
    })
}

fn resolve_source(s: Source, ctx: &ResolveContext) -> anyhow::Result<Source> {
    Ok(Source {
        variant: resolve_variant(s.variant, ctx)?,

        version_overrides: s
            .version_overrides
            .map(|vos| {
                vos.into_iter()
                    .map(|vo| resolve_version_override(vo, ctx))
                    .collect::<anyhow::Result<_>>()
            })
            .transpose()?,

        bin: s.bin.map(|b| public::parse_template(b, ctx)).transpose()?,

        ..s
    })
}

fn resolve_variant(v: SourceVariant, ctx: &ResolveContext) -> anyhow::Result<SourceVariant> {
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
                .map(|a| resolve_asset(a, ctx))
                .collect::<anyhow::Result<_>>()?,
        )),

        SourceVariant::Download(downloads) => {
            Ok(SourceVariant::Download(resolve_downloads(downloads, ctx)?))
        }

        SourceVariant::Build(builds) => Ok(SourceVariant::Build(
            builds
                .into_iter()
                .map(|b| resolve_build(b, ctx))
                .collect::<anyhow::Result<_>>()?,
        )),
    }
}

fn resolve_version_override(
    vo: VersionOverride,
    ctx: &ResolveContext,
) -> anyhow::Result<VersionOverride> {
    Ok(VersionOverride {
        variant: resolve_variant(vo.variant, ctx)?,
        ..vo
    })
}

fn resolve_asset(a: Asset, ctx: &ResolveContext) -> anyhow::Result<Asset> {
    Ok(Asset {
        files: a
            .files
            .into_iter()
            .map(|f| public::parse_template(f, ctx))
            .collect::<anyhow::Result<_>>()?,

        bin: a
            .bin
            .map(|bin| bin.try_map(|b| public::parse_template(b, ctx)))
            .transpose()?,

        variables: a
            .variables
            .into_iter()
            .map(|(k, v)| {
                Ok((
                    public::parse_template(k, ctx)?,
                    v.try_map(|vars| public::parse_template(vars, ctx))?,
                ))
            })
            .collect::<anyhow::Result<_>>()?,

        ..a
    })
}

fn resolve_downloads(d: Downloads, ctx: &ResolveContext) -> anyhow::Result<Downloads> {
    match d {
        Downloads::Simple { file } => Ok(Downloads::Simple {
            file: public::parse_template(file, ctx)?,
        }),

        Downloads::Detailed(downloads) => Ok(Downloads::Detailed(
            downloads
                .into_iter()
                .map(|d| resolve_download(d, ctx))
                .collect::<anyhow::Result<_>>()?,
        )),
    }
}

fn resolve_download(d: Download, ctx: &ResolveContext) -> anyhow::Result<Download> {
    Ok(Download {
        files: public::parse_template_hashmap(d.files, ctx)?,
        bin: d.bin.map(|b| public::parse_template(b, ctx)).transpose()?,

        ..d
    })
}

fn resolve_build(b: Build, ctx: &ResolveContext) -> anyhow::Result<Build> {
    Ok(Build {
        bin: b
            .bin
            .map(|bin| bin.try_map(|b| public::parse_template(b, ctx)))
            .transpose()?,

        env: b
            .env
            .map(|e| public::parse_template_hashmap(e, ctx))
            .transpose()?,

        extra: public::parse_template_hashmap(b.extra, ctx)?,
        ..b
    })
}
