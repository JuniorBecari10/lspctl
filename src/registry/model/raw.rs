// TODO: remove this as soon as they are actually used
#![allow(unused)]

use std::collections::HashMap;

use serde::Deserialize;

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
}

// TODO: add Asset
#[derive(Deserialize, Debug, Clone)]
struct RawSource {
    id: String, // (purl)
    extra_packages: Option<Vec<String>>,

    #[serde(rename = "asset")]
    assets: Option<OneOrMany<RawAsset>>,
    download: Option<RawDownloads>,
    build: Option<OneOrMany<RawBuild>>,
    supported_platforms: Option<Vec<String>>,
    bin: Option<String>, // for js-debug-adapter (edge case)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum RawDownloads {
    Simple { file: String },
    Detailed(OneOrMany<RawDownload>),
}

#[derive(Deserialize, Debug, Clone)]
struct RawDownload {
    target: Option<OneOrMany<String>>,
    files: HashMap<String, String>,
    bin: Option<String>, // this may change with a Mason update
}

// this has 'bool staged' and
// 'erlang_ls' and 'els_dap' are their own fields in these packages
#[derive(Deserialize, Debug, Clone)]
struct RawBuild {
    run: String,
    target: Option<OneOrMany<String>>,
    bin: Option<OneOrMap>,
    env: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, Clone)]
struct RawAsset {
    target: Option<OneOrMany<String>>,
    file: OneOrMany<String>,
    bin: Option<OneOrMap>,
}

// ---

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(super) enum OneOrMany<T: Clone> {
    One(T),
    Many(Vec<T>),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum OneOrMap {
    One(String),
    Map(HashMap<String, String>),
}
