use std::{collections::HashMap, fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::{io, paths, registry::model::SourceVariant};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub schema_version: u32,
    pub installed: HashMap<String, InstalledPackage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstalledPackage {
    pub version: String,
    pub install_kind: SourceVariant,
}

impl State {
    pub fn load() -> anyhow::Result<Self> {
        let mut contents = Vec::new();
        File::open(paths::state_file())?.read_to_end(&mut contents)?;

        let state: Self = serde_json::from_slice(&contents)?;
        Ok(state)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let contents = serde_json::to_vec_pretty(self)?;
        io::write_file_atomic_contents(&paths::state_file(), &contents, true)
    }
}
