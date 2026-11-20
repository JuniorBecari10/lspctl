pub mod context;
pub mod interpreter;
pub mod public;
pub mod select;

use crate::registry::{
    model::{Asset, Build, Download, Downloads, Entry, Source, Variant},
    parser::template::context::ResolveContext,
};

// just for documenting and clarity purposes
type PreResolvedEntry = Entry;

// pre-resolving means all templates replaced with actual data.
// the type is the same, but it now holds pre-resolved data.
// yes, 'entry.bin' is resolved here
pub fn pre_resolve_entry(e: Entry, ctx: &ResolveContext) -> anyhow::Result<PreResolvedEntry> {
    Ok(Entry {
        bin: e
            .bin
            .map(|b| public::parse_template_hashmap(b, ctx))
            .transpose()?,

        source: pre_resolve_source(e.source, ctx)?,
        ..e
    })
}

fn pre_resolve_source(s: Source, ctx: &ResolveContext) -> anyhow::Result<Source> {
    Ok(Source {
        variant: pre_resolve_variant(s.variant, ctx)?,
        bin: s.bin.map(|b| public::parse_template(b, ctx)).transpose()?,

        ..s
    })
}

fn pre_resolve_variant(v: Variant, ctx: &ResolveContext) -> anyhow::Result<Variant> {
    match v {
        Variant::PackageManager {
            manager,
            extra_packages,
        } => Ok(Variant::PackageManager {
            manager,
            extra_packages,
        }),

        Variant::Asset(assets) => Ok(Variant::Asset(
            assets
                .into_iter()
                .map(|a| pre_resolve_asset(a, ctx))
                .collect::<anyhow::Result<_>>()?,
        )),

        Variant::Download(downloads) => {
            Ok(Variant::Download(pre_resolve_downloads(downloads, ctx)?))
        }

        Variant::Build(builds) => Ok(Variant::Build(
            builds
                .into_iter()
                .map(|b| pre_resolve_build(b, ctx))
                .collect::<anyhow::Result<_>>()?,
        )),
    }
}

fn pre_resolve_asset(a: Asset, ctx: &ResolveContext) -> anyhow::Result<Asset> {
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

fn pre_resolve_downloads(d: Downloads, ctx: &ResolveContext) -> anyhow::Result<Downloads> {
    match d {
        Downloads::Simple { file } => Ok(Downloads::Simple {
            file: public::parse_template(file, ctx)?,
        }),

        Downloads::Detailed(downloads) => Ok(Downloads::Detailed(
            downloads
                .into_iter()
                .map(|d| pre_resolve_download(d, ctx))
                .collect::<anyhow::Result<_>>()?,
        )),
    }
}

fn pre_resolve_download(d: Download, ctx: &ResolveContext) -> anyhow::Result<Download> {
    Ok(Download {
        files: public::parse_template_hashmap(d.files, ctx)?,
        bin: d.bin.map(|b| public::parse_template(b, ctx)).transpose()?,

        ..d
    })
}

fn pre_resolve_build(b: Build, ctx: &ResolveContext) -> anyhow::Result<Build> {
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
