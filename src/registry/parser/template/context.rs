use serde::Serialize;
use serde_json::{Map, Value};

use crate::registry::{
    model::{Platform, Source, Variant},
    parser::template::select,
};

// Holds the best variant for your system plus some needed metadata, since this is what
// will be passed around and not the ResolvedEntry for that package.
#[derive(Serialize)]
pub struct ResolveContext {
    pub version: String,
    pub variant: ResolveVariant,
    pub bin: Option<String>, // Source.bin: the js-debug-adapter edge case
}

// The best variant chosen for this platform for the specific package you want to install.
// For example, there are a lot of Assets for some package and this holds the best one suitable
// for your system.
#[derive(Serialize)]
pub enum ResolveVariant {
    PackageManager,
    Asset(Value),
    Download(Value),
    Build(Value),
}

pub fn build_context(source: &Source, host: &Platform) -> anyhow::Result<ResolveContext> {
    let variant =
        match &source.variant {
            Variant::PackageManager { .. } => ResolveVariant::PackageManager,

            Variant::Asset(_) => ResolveVariant::Asset(serde_json::to_value(
                select::select_asset(&source.variant, host)?,
            )?),

            Variant::Download(_) => ResolveVariant::Download(serde_json::to_value(
                select::select_download(&source.variant, host)?,
            )?),

            Variant::Build(_) => ResolveVariant::Build(serde_json::to_value(
                select::select_build(&source.variant, host)?,
            )?),
        };

    Ok(ResolveContext {
        version: source.purl.version.clone(),
        variant,
        bin: source.bin.clone(),
    })
}

impl ResolveContext {
    pub fn to_template_json(&self) -> Value {
        let mut map = Map::new();
        map.insert("version".into(), serde_json::json!(self.version));

        if let Some(bin) = &self.bin {
            map.insert("bin".into(), serde_json::json!(bin));
        }

        match &self.variant {
            ResolveVariant::Asset(a) => {
                map.insert(
                    "asset".into(),
                    serde_json::to_value(a).unwrap_or(Value::Null),
                );
            }
            ResolveVariant::Download(d) => {
                map.insert(
                    "download".into(),
                    serde_json::to_value(d).unwrap_or(Value::Null),
                );
            }
            ResolveVariant::Build(b) => {
                map.insert(
                    "build".into(),
                    serde_json::to_value(b).unwrap_or(Value::Null),
                );
            }
            ResolveVariant::PackageManager => {} // no asset/download/build key; nothing to template against
        }

        Value::Object(map)
    }
}
