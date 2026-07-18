mod util;

use anyhow::anyhow;
use packageurl::PackageUrl;

use crate::registry::{
    model::{
        Entry, RawEntry, RawRegistry, RawSource, RawSourceVariant, Registry, Source, SourceVariant,
    },
    parser::util::get_platform,
};

pub fn parse_registry(raw: RawRegistry) -> anyhow::Result<Registry> {
    dbg!(&raw);
    Ok(Registry(
        raw.0
            .into_iter()
            .map(parse_entry)
            .collect::<anyhow::Result<Vec<_>>>()?,
    ))
}

fn parse_entry(entry: RawEntry) -> anyhow::Result<Entry> {
    Ok(Entry {
        name: entry.name,
        description: entry.description,
        homepage: entry.homepage,
        licenses: entry.licenses,
        languages: entry.languages,
        categories: entry.categories,
        source: parse_source(entry.source)?,
        bin: entry.bin,
        deprecation: entry.deprecation,
    })
}

fn parse_source(raw: RawSource) -> anyhow::Result<Source> {
    Ok(Source {
        purl: PackageUrl::new(raw.id, "purl")?.try_into()?,
        variant: get_variant(raw.variant)?,
        supported_platforms: raw
            .supported_platforms
            .map(|v| {
                v.iter()
                    .map(|p| get_platform(p).ok_or_else(|| anyhow!("Invalid platform: {}", p)))
                    .collect::<anyhow::Result<Vec<_>>>()
            })
            .transpose()?,
        bin: raw.bin,
    })
}

fn get_variant(variant: RawSourceVariant) -> anyhow::Result<SourceVariant> {}
