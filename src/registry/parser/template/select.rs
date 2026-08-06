use crate::registry::model::{Asset, Build, Download, Downloads, Platform, Variant};

pub trait Targets {
    fn targets(&self) -> &[Platform];
}

impl Targets for Asset {
    fn targets(&self) -> &[Platform] {
        &self.targets
    }
}

impl Targets for Download {
    fn targets(&self) -> &[Platform] {
        &self.targets
    }
}

impl Targets for Build {
    fn targets(&self) -> &[Platform] {
        &self.targets
    }
}

// ---

fn select_matching<'a, T: Targets>(
    items: &'a [T],
    host: &Platform,
    label: &str,
) -> anyhow::Result<&'a T> {
    items
        .iter()
        .filter(|item| item.targets().iter().any(|p| p.matches(host)))
        .max_by_key(|item| {
            item.targets()
                .iter()
                .map(Platform::specificity)
                .max()
                .unwrap_or(0)
        })
        .ok_or_else(|| anyhow::anyhow!("No {label} entry matches platform {host}"))
}

pub fn select_asset<'a>(variant: &'a Variant, host: &Platform) -> anyhow::Result<&'a Asset> {
    let Variant::Asset(assets) = variant else {
        anyhow::bail!("Source has no asset variant (got '{variant}')");
    };

    select_matching(assets, host, "asset")
}

pub fn select_download(variant: &Variant, host: &Platform) -> anyhow::Result<serde_json::Value> {
    let Variant::Download(downloads) = variant else {
        anyhow::bail!("Source has no download variant (got '{variant}')");
    };

    match downloads {
        Downloads::Simple { file } => Ok(serde_json::json!({ "file": file })),

        Downloads::Detailed(list) => {
            let selected = select_matching(list, host, "download")?;
            Ok(serde_json::to_value(selected)?)
        }
    }
}

pub fn select_build<'a>(variant: &'a Variant, host: &Platform) -> anyhow::Result<&'a Build> {
    let Variant::Build(builds) = variant else {
        anyhow::bail!("Source has no build variant (got '{variant}')");
    };

    select_matching(builds, host, "build")
}
