use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{io, paths};

pub fn link_npm(
    bins: Vec<&String>,
    tmp_pkg_path: &Path,
) -> anyhow::Result<HashMap<String, PathBuf>> {
    let files = io::list_files(&tmp_pkg_path.join("node_modules").join(".bin"))?;
    let map = HashMap::new();

    for file in files {
        let name = file
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or_else(|| anyhow::anyhow!("Invalid file path: '{}'", file.display()))?;

        if !bins.contains(&&name) {
            continue;
        }

        io::link_files(&file, &paths::bin_dir().join(name))?;
    }

    Ok(map)
}
