use std::path::PathBuf;

use crate::consts;

const STATE_FILE_NAME: &str = "state.json";

fn root_dir() -> PathBuf {
    dirs::data_local_dir()
        .expect("Could not determine home directory.")
        .join(consts::APP_NAME)
}

pub fn registry_dir() -> PathBuf {
    root_dir().join(consts::REGISTRY_DIR)
}

pub fn packages_dir() -> PathBuf {
    root_dir().join(consts::PACKAGES_DIR)
}

pub fn bin_dir() -> PathBuf {
    root_dir().join(consts::BIN_DIR)
}

pub fn state_file() -> PathBuf {
    root_dir().join(STATE_FILE_NAME)
}
