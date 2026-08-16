use crate::{
    error, fatal,
    registry::{
        model::{
            Downloads, Entry, Platform, ResolvedDownloads, ResolvedEntry, ResolvedSource,
            ResolvedVariant, Variant,
        },
        parser::template::select::Targets,
    },
};

pub fn merge_context(entry: Entry, host: &Platform) -> ResolvedEntry {
    let variant = match entry.source.variant {
        Variant::PackageManager {
            manager,
            extra_packages,
        } => ResolvedVariant::PackageManager {
            manager,
            extra_packages,
        },

        Variant::Asset(assets) => {
            let asset = select_owned(assets, host, "asset");
            ResolvedVariant::Asset(asset)
        }

        Variant::Download(downloads) => ResolvedVariant::Download(match downloads {
            Downloads::Simple { file } => ResolvedDownloads::Simple { file },
            Downloads::Detailed(list) => {
                let download = select_owned(list, host, "download");
                ResolvedDownloads::Detailed(download)
            }
        }),

        Variant::Build(builds) => {
            let build = select_owned(builds, host, "build");
            ResolvedVariant::Build(build)
        }
    };

    let source = ResolvedSource {
        purl: entry.source.purl,
        variant,
        supported_platforms: entry.source.supported_platforms,
        bin: entry.source.bin, // already resolved
    };

    ResolvedEntry {
        name: entry.name,
        description: entry.description,
        homepage: entry.homepage,
        languages: entry.languages,
        licenses: entry.licenses,
        categories: entry.categories,
        source,
        bin: entry.bin, // already resolved
        deprecation: entry.deprecation,
    }
}

fn select_owned<T: Targets>(items: Vec<T>, host: &Platform, label: &str) -> T {
    let index = items
        .iter()
        .enumerate()
        .filter(|(_, item)| item.matches(host))
        .max_by_key(|(_, item)| item.specificity())
        .map(|(i, _)| i)
        .unwrap_or_else(|| {
            fatal!(
                "No {label} entry matches platform '{host}'. \
                 'build_context' should have already caught this."
            )
        });

    let mut items = items;
    items.swap_remove(index)
}
