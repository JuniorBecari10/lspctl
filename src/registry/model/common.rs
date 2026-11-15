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

#[derive(Deserialize, Serialize, Debug)]
#[allow(clippy::upper_case_acronyms)] // to avoid renaming with serde
pub enum Category {
    Compiler,
    DAP,
    Formatter,
    LSP,
    Linter,
    Runtime,
}
