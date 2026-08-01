use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    global, io, paths,
    registry::model::{Entry, InstallKind},
};

// only change this after launch and after introducing a breaking change to the registry state.
const SCHEMA_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub schema_version: u32,
    pub installed: HashMap<String, InstalledPackage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstalledPackage {
    pub version: String,
    pub install_kind: InstallKind,
    pub bin: HashMap<String, PathBuf>,
    pub install_time: String,
}

impl State {
    pub fn load() -> anyhow::Result<Self> {
        let path = paths::state_file();

        // first run should not be an error
        if !path.exists() {
            return Ok(Self {
                schema_version: SCHEMA_VERSION,
                installed: HashMap::new(),
            });
        }

        let mut contents = Vec::new();
        File::open(paths::state_file())?.read_to_end(&mut contents)?;

        let state: Self = serde_json::from_slice(&contents)?;
        match state.schema_version {
            SCHEMA_VERSION => Ok(state),
            older => anyhow::bail!(
                "State file has an schema version 'v{older}', which is older than the current 'v{SCHEMA_VERSION}' version."
            ),
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let contents = serde_json::to_vec_pretty(self)?;
        io::write_file_atomic_contents(&paths::state_file(), &contents, true)
    }

    pub fn add_entry(&mut self, e: &Entry, bin: HashMap<String, PathBuf>) {
        self.installed.insert(
            e.name.clone(),
            InstalledPackage {
                version: e.source.purl.version.clone(),
                install_kind: e.source.purl.kind,
                bin,
                install_time: global::time_now(),
            },
        );
    }

    pub fn package_exists(&self, name: &str) -> bool {
        self.installed.contains_key(name)
    }
}
