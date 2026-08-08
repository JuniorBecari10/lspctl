use std::path::PathBuf;

use crate::{consts, log::Fatal, registry};

consts::consts!(
    BIN_DIR = "bin",
    TMP_DIR = "tmp",
    REGISTRY_DIR = "registry",
    PACKAGES_DIR = "packages",
    STATE_FILE = "state.json"
);

fn root_dir() -> PathBuf {
    dirs::data_local_dir()
        .fatal("Could not determine home directory.")
        .join(consts::APP_NAME)
}

pub fn registry_dir() -> PathBuf {
    root_dir().join(REGISTRY_DIR)
}

pub fn packages_dir() -> PathBuf {
    root_dir().join(PACKAGES_DIR)
}

pub fn bin_dir() -> PathBuf {
    root_dir().join(BIN_DIR)
}

pub fn tmp_dir() -> PathBuf {
    root_dir().join(TMP_DIR)
}

// ---

pub fn state_file() -> PathBuf {
    root_dir().join(STATE_FILE)
}

pub fn registry_file() -> PathBuf {
    registry_dir().join(registry::REGISTRY_FILE)
}

// ---

pub fn tmp_package_folder(name: &str, version: &str) -> PathBuf {
    tmp_dir().join(name).join(version)
}
