use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::registry::model::{AssetVars, OneOrMap, common::Deprecation};

#[derive(Deserialize, Debug)]
pub struct RawRegistry(pub Vec<RawEntry>);

#[derive(Deserialize, Debug)]
pub struct RawEntry {
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub licenses: Vec<String>,
    pub languages: Vec<String>,
    pub categories: Vec<String>,
    pub source: RawSource,
    pub bin: Option<HashMap<String, String>>,
    pub deprecation: Option<Deprecation>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RawSource {
    pub id: String, // (purl)
    #[serde(flatten)]
    pub variant: Option<RawSourceVariant>,
    pub supported_platforms: Option<Vec<String>>,
    pub bin: Option<String>, // for js-debug-adapter (edge case)
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum RawSourceVariant {
    ExtraPackages { extra_packages: Vec<String> },
    Asset { asset: OneOrMany<RawAsset> },
    Download { download: RawDownloads },
    Build { build: OneOrMany<RawBuild> },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum RawDownloads {
    Simple { file: String },
    Detailed(OneOrMany<RawDownload>),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RawDownload {
    pub target: Option<OneOrMany<String>>,
    pub files: HashMap<String, String>,
    pub bin: Option<String>, // this may change with a Mason update
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RawBuild {
    pub run: String,
    pub target: Option<OneOrMany<String>>,
    pub bin: Option<OneOrMap>,
    pub env: Option<HashMap<String, String>>,
    pub staged: Option<bool>,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RawAsset {
    pub target: Option<OneOrMany<String>>,
    pub file: OneOrMany<String>,
    pub bin: Option<OneOrMap>,
    #[serde(flatten)]
    pub variables: HashMap<String, AssetVars>,
}

// ---

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}
