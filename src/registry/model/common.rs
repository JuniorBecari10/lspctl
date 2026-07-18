use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Deprecation {
    since: String,
    message: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OneOrMap {
    One(String),
    Map(HashMap<String, String>),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum AssetExtra {
    Path(String),
    Nested(HashMap<String, String>),
}
