use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::registry::model::{Asset, Build, Deprecation, Download, PackageManager, Platform, Purl};

#[derive(Debug, Serialize)]
pub struct ResolvedEntry {
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub licenses: Vec<String>,
    pub languages: Vec<String>,
    pub categories: Vec<String>,
    pub source: ResolvedSource,
    pub bin: Option<HashMap<String, String>>,
    pub deprecation: Option<Deprecation>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedSource {
    pub purl: Purl,
    pub variant: ResolvedVariant,
    pub supported_platforms: Vec<Platform>,
    pub version_overrides: Option<Vec<ResolvedVersionOverride>>,
    pub bin: Option<String>, // for js-debug-adapter (edge case)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolvedVariant {
    PackageManager {
        manager: PackageManager,
        extra_packages: Vec<String>,
    },
    Asset(Asset),
    Download(ResolvedDownloads),
    Build(Build),
}

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedVersionOverride {
    pub constraint: String,
    pub id: String,
    pub variant: ResolvedVariant,
    pub supported_platforms: Vec<Platform>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolvedDownloads {
    Simple { file: String },
    Detailed(Download),
}
