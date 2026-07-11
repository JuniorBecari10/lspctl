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
    id: String,
    extra_packages: Option<Vec<String>>,

    #[serde(rename = "asset")]
    assets: Option<OneOrMany<Asset>>,
    download: Option<Downloads>,
    build: Option<OneOrMany<Build>>,
    supported_platforms: Option<Vec<String>>,
    bin: Option<String>, // for js-debug-adapter (edge case)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Downloads {
    Simple { file: String },
    Detailed(OneOrMany<Download>),
}

#[derive(Deserialize, Debug, Clone)]
struct Download {
    target: Option<OneOrMany<String>>,
    files: HashMap<String, String>,
    bin: Option<String>, // this may change with a Mason update
}

// this has 'bool staged' and
// 'erlang_ls' and 'els_dap' are their own fields in these packages
#[derive(Deserialize, Debug, Clone)]
struct Build {
    run: String,
    target: Option<OneOrMany<String>>,
    bin: Option<OneOrMap>,
    env: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, Clone)]
struct Asset {
    target: Option<OneOrMany<String>>,
    file: OneOrMany<String>,
    bin: Option<OneOrMap>,
}

// ---

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum OneOrMany<T: Clone> {
    One(T),
    Many(Vec<T>),
}

// TODO: optimize so that this is moved rather than cloned
impl<T: Clone> OneOrMany<T> {
    pub fn to_vec(&self) -> Vec<T> {
        match self {
            OneOrMany::One(t) => vec![t.clone()],
            OneOrMany::Many(t) => t.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum OneOrMap {
    One(String),
    Map(HashMap<String, String>),
}
