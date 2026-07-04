use std::path::PathBuf;

use crate::consts;

pub fn root_dir() -> PathBuf {
    dirs::data_local_dir()
        .expect("Could not determine home directory")
        .join("lspctl")
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
