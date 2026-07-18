// TODO: remove this as soon as they are actually used
#![allow(unused)]

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
    pub extra_packages: Option<Vec<String>>, // any package manager
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
    target: Option<OneOrMany<String>>,
    files: HashMap<String, String>,
    bin: Option<String>, // this may change with a Mason update
}

#[derive(Deserialize, Debug)]
pub struct RawBuild {
    run: String,
    target: Option<OneOrMany<String>>,
    bin: Option<OneOrMap>,
    env: Option<HashMap<String, String>>,

    staged: Option<bool>,
    erlang_ls: Option<String>,
    els_dap: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RawAsset {
    target: Option<OneOrMany<String>>,
    file: OneOrMany<String>,
    bin: Option<OneOrMap>,
    #[serde(flatten)]
    extra: HashMap<String, AssetExtra>,
}

// ---

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}
