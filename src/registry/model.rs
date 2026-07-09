use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Release {
    pub assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub name: String,

    #[serde(rename = "browser_download_url")]
    pub url: String,
}

// ---

#[derive(Deserialize, Debug)]
pub struct Registry(Vec<Entry>);

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub licenses: Vec<String>,
    pub languages: Vec<String>,
    pub categories: Vec<String>,
    pub source: Source,
    pub bin: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct Source {}
