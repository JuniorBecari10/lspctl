use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Deprecation {
    pub since: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum OneOrMap {
    One(String),
    Map(HashMap<String, String>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum AssetVars {
    Path(String),
    Nested(HashMap<String, String>),
}
