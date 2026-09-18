use std::{fs, path::Path};

use anyhow::Context;

use crate::{disk, log::Format};

pub fn rewrite_embedded_paths(
    dir: &Path,
    old_prefix: &Path,
    new_prefix: &Path,
) -> anyhow::Result<()> {
    let old = old_prefix.to_string_lossy();
    let new = new_prefix.to_string_lossy();

    for file in disk::list_files(dir)? {
        let contents = fs::read_to_string(&file)
            .with_context(|| format!("Failed to read {}", file.display().quote()))?;

        if contents.contains(old.as_ref()) {
            let fixed = contents.replace(old.as_ref(), new.as_ref());
            fs::write(&file, fixed)
                .with_context(|| format!("Failed to rewrite {}", file.display().quote()))?;
        }
    }

    Ok(())
}
