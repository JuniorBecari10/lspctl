use std::{
    fs,
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use tempfile::NamedTempFile;

use crate::paths;

/// creates a new temporary file in lspctl/tmp.
/// requires tmp to exist. error if not.
/// it doesn't create it because it is expected to exist at this point.
pub fn new_temp() -> anyhow::Result<NamedTempFile> {
    Ok(NamedTempFile::new_in(paths::tmp_dir())?)
}

// this function must be atomic
fn persist(temp: NamedTempFile, p: &Path, replace: bool) -> anyhow::Result<()> {
    let res = if replace {
        temp.persist(p)
    } else {
        temp.persist_noclobber(p)
    };

    match res {
        Ok(_) => Ok(()),
        Err(e) if e.error.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(e.error.into()),
    }
}

pub fn write_file_atomic_contents(p: &Path, contents: &[u8], replace: bool) -> anyhow::Result<()> {
    let mut temp = new_temp()?;
    temp.write_all(contents)?;
    persist(temp, p, replace)
}

pub fn link_files(from: &Path, to: &Path) -> anyhow::Result<()> {
    if !from.exists() {
        anyhow::bail!("link target does not exist: '{}'", from.display());
    }

    let real_target = fs::canonicalize(from)
        .with_context(|| format!("failed to resolve real path of '{}'", from.display()))?;

    if !real_target.is_file() {
        anyhow::bail!(
            "resolved link target is not a regular file: '{}'",
            real_target.display()
        );
    }

    if to.exists() || to.is_symlink() {
        fs::remove_file(to)
            .with_context(|| format!("failed to remove existing link at '{}'", to.display()))?;
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(&real_target, to).with_context(|| {
        format!(
            "failed to symlink '{}' -> '{}'",
            to.display(),
            real_target.display()
        )
    })?;

    #[cfg(windows)]
    fs::copy(&real_target, to).with_context(|| {
        format!(
            "failed to copy '{}' -> '{}'",
            real_target.display(),
            link_path.display()
        )
    })?;

    Ok(())
}

pub fn list_files(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

fn make_writable(path: &Path) -> anyhow::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = fs::metadata(path)?.permissions();
        let mode = perms.mode();

        if mode & 0o200 == 0 {
            perms.set_mode(mode | 0o200);
            fs::set_permissions(path, perms)
                .with_context(|| format!("Failed to chmod {}", path.display()))?;
        }
    }

    #[cfg(windows)]
    {
        let mut perms = fs::metadata(path)?.permissions();

        if perms.readonly() {
            perms.set_readonly(false);
            fs::set_permissions(path, perms).with_context(|| {
                format!("Failed to clear read-only attribute on {}", path.display())
            })?;
        }
    }

    Ok(())
}

pub fn make_writable_recursive(dir: &Path) -> anyhow::Result<()> {
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry?;

        if entry.file_type().is_symlink() {
            continue;
        }

        make_writable(entry.path())?;
    }
    Ok(())
}
