use serde::Serialize;

use crate::registry::{
    model::{Platform, Source, SourceVariant},
    parser::template::select::select_asset,
};

#[derive(Serialize)]
pub struct ResolvedContext {
    pub version: String,
    pub asset: Option<serde_json::Value>, // *one* selected Asset, not the tagged array
    pub bin: Option<String>,              // Source.bin: the js-debug-adapter edge case
}

pub fn build_context(source: &Source, host: &Platform) -> anyhow::Result<ResolvedContext> {
    let asset = match &source.variant {
        Some(SourceVariant::Asset(_)) => {
            Some(serde_json::to_value(select_asset(&source.variant, host)?)?)
        }

        _ => None, // package uses Build/Download/ExtraPackages, no asset templates apply
    };

    Ok(ResolvedContext {
        version: source.purl.version.clone(),
        asset,
        bin: source.bin.clone(),
    })
}
