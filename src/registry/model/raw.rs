// TODO: remove this as soon as they are actually used
#![allow(unused)]

use std::collections::HashMap;

use serde::Deserialize;

use crate::registry::model::common::Deprecation;

#[derive(Deserialize, Debug)]
pub struct RawRegistry(Vec<RawEntry>);

#[derive(Deserialize, Debug)]
struct RawEntry {
    name: String,
    description: String,
    homepage: String,
    licenses: Vec<String>,
    languages: Vec<String>,
    categories: Vec<String>,
    source: RawSource,
    bin: Option<HashMap<String, String>>,
    deprecation: Option<Deprecation>,
}

#[derive(Deserialize, Debug)]
struct RawSource {
    id: String, // (purl)

    #[serde(flatten)]
    variant: RawSourceVariant,
    supported_platforms: Option<Vec<String>>,
    version_overrides: Option<Vec<RawVersionOverride>>,
}

#[derive(Deserialize, Debug)]
struct RawSourceVariant {
    // one and only one of these will be present at once
    #[serde(rename = "asset")]
    assets: Option<OneOrMany<RawAsset>>, // github
    extra_packages: Option<Vec<String>>, // any package manager
    download: Option<RawDownloads>,      // generic / openvsx
    build: Option<OneOrMany<RawBuild>>,  // build
    bin: Option<String>,                 // for js-debug-adapter (edge case)
}

#[derive(Deserialize, Debug)]
struct RawVersionOverride {
    constraint: String,
    id: String,

    #[serde(flatten)]
    variant: RawSourceVariant,
    supported_platforms: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RawDownloads {
    Simple { file: String },
    Detailed(OneOrMany<RawDownload>),
}

#[derive(Deserialize, Debug)]
struct RawDownload {
    target: Option<OneOrMany<String>>,
    files: HashMap<String, String>,
    bin: Option<String>, // this may change with a Mason update
}

#[derive(Deserialize, Debug)]
struct RawBuild {
    run: String,
    target: Option<OneOrMany<String>>,
    bin: Option<OneOrMap>,
    env: Option<HashMap<String, String>>,

    staged: Option<bool>,
    erlang_ls: Option<String>,
    els_dap: Option<String>,
}

#[derive(Deserialize, Debug)]
struct RawAsset {
    target: Option<OneOrMany<String>>,
    file: OneOrMany<String>,
    bin: Option<OneOrMap>,
    #[serde(flatten)]
    extra: HashMap<String, AssetExtra>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum AssetExtra {
    Path(String),
    Nested(HashMap<String, String>),
}

// ---

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub(super) enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum OneOrMap {
    One(String),
    Map(HashMap<String, String>),
}
