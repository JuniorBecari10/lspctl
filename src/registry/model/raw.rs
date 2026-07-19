use std::collections::HashMap;

use serde::Deserialize;

use crate::registry::model::{AssetExtra, OneOrMap, common::Deprecation};

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

#[derive(Deserialize, Debug)]
pub struct RawSource {
    pub id: String, // (purl)

    #[serde(flatten)]
    pub variant: RawSourceVariant,
    pub supported_platforms: Option<Vec<String>>,
    pub version_overrides: Option<Vec<RawVersionOverride>>,
    pub bin: Option<String>, // for js-debug-adapter (edge case)
}

#[derive(Deserialize, Debug)]
pub struct RawSourceVariant {
    // one and only one of these will be present at once
    // TODO: enforce this rule
    pub extra_packages: Option<Vec<String>>, // any package manager. kind must be package manager for this to be Some().
    #[serde(rename = "asset")]
    pub assets: Option<OneOrMany<RawAsset>>, // github
    pub download: Option<RawDownloads>,      // generic / openvsx
    pub build: Option<OneOrMany<RawBuild>>,  // build
}

#[derive(Deserialize, Debug)]
pub struct RawVersionOverride {
    pub constraint: String,
    pub id: String,

    #[serde(flatten)]
    pub variant: RawSourceVariant,
    pub supported_platforms: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RawDownloads {
    Simple { file: String },
    Detailed(OneOrMany<RawDownload>),
}

#[derive(Deserialize, Debug)]
pub struct RawDownload {
    pub target: Option<OneOrMany<String>>,
    pub files: HashMap<String, String>,
    pub bin: Option<String>, // this may change with a Mason update
}

#[derive(Deserialize, Debug)]
pub struct RawBuild {
    pub run: String,
    pub target: Option<OneOrMany<String>>,
    pub bin: Option<OneOrMap>,
    pub env: Option<HashMap<String, String>>,
    pub staged: Option<bool>,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct RawAsset {
    pub target: Option<OneOrMany<String>>,
    pub file: OneOrMany<String>,
    pub bin: Option<OneOrMap>,
    #[serde(flatten)]
    pub extra: HashMap<String, AssetExtra>,
}

// ---

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}
