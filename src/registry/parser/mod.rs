mod util;

use packageurl::PackageUrl;

use crate::registry::{
    model::{
        Entry, InstallKind, Purl, RawEntry, RawRegistry, RawSource, RawSourceVariant,
        RawVersionOverride, Registry, Source, SourceVariant, VersionOverride,
    },
    parser::util::convert_platforms,
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

fn parse_variant(variant: RawSourceVariant, kind: InstallKind) -> anyhow::Result<SourceVariant> {
    if kind.is_package_manager() {}
}

fn parse_version_override(
    version: RawVersionOverride,
    kind: InstallKind,
) -> anyhow::Result<VersionOverride> {
    Ok(VersionOverride {
        constraint: version.constraint,
        id: version.id,
        variant: parse_variant(version.variant, kind)?,
        supported_platforms: convert_platforms(version.supported_platforms)?,
    })
}
