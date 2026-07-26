use crate::registry::model::{Asset, Platform, SourceVariant};

pub fn select_asset<'a>(
    variant: &'a Option<SourceVariant>,
    host: &Platform,
) -> anyhow::Result<&'a Asset> {
    let Some(SourceVariant::Asset(assets)) = variant else {
        anyhow::bail!("source has no asset variant (got {variant:?})");
    };

    assets
        .iter()
        .filter(|a| a.targets.iter().any(|p| p.matches(host)))
        .max_by_key(|a| {
            a.targets
                .iter()
                .map(Platform::specificity)
                .max()
                .unwrap_or(0)
        })
        .ok_or_else(|| anyhow::anyhow!("No asset entry matches platform {host:?}"))
}
