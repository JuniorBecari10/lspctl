use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Release {
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseAsset {
    pub name: String,

    #[serde(rename = "browser_download_url")]
    pub url: String,
}

// ---
// TODO: add these in another file

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
#[derive(Deserialize, Debug)]
struct RawSource {
    id: String,
    extra_packages: Option<Vec<String>>,
    // download: Option<Vec<String>>, // not supported for now
    build: Option<Options<Build>>,
    supported_platforms: Option<Vec<String>>,
    bin: Option<String>, // for js-debug-adapter (edge case)
}

// this has 'bool staged' and
// 'erlang_ls' and 'els_dap' are their own fields in these packages
#[derive(Deserialize, Debug)]
struct Build {
    run: String,
    target: Option<Options<String>>,
    bin: Option<MapOptions>,
    env: Option<HashMap<String, String>>,
}

// ---

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Options<T> {
    Array(Vec<T>),
    Single(T),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum MapOptions {
    Map(HashMap<String, String>),
    Single(String),
}
