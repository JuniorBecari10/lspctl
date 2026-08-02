use serde::Serialize;

use crate::registry::{
    model::{Platform, Source, Variant},
    parser::template::select::select_asset,
};

#[derive(Serialize)]
pub struct ResolveContext {
    pub version: String,
    pub variant: ResolveVariant,
    pub bin: Option<String>, // Source.bin: the js-debug-adapter edge case
}

#[derive(Serialize)]
enum ResolveVariant {
    Asset(serde_json::Value),
    Download(serde_json::Value),
    Build(serde_json::Value),
}

pub fn build_context(source: &Source, host: &Platform) -> anyhow::Result<ResolveContext> {
    let variant = match &source.variant {
        Variant::PackageManager {
            manager,
            extra_packages: _,
        } => todo!(),

        Variant::Asset(_) => {
            ResolveVariant::Asset(serde_json::to_value(select_asset(&source.variant, host)?)?)
        }

        Variant::Download(downloads) => todo!(),
        Variant::Build(builds) => todo!(),
    };

    Ok(ResolveContext {
        version: source.purl.version.clone(),
        variant,
        bin: source.bin.clone(),
    })
}
