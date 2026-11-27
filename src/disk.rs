use std::{
    fs,
    io::{self, ErrorKind, Read, Seek, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use indicatif::ProgressBar;
use tempfile::NamedTempFile;
use ureq::BodyReader;
use zip::ZipArchive;

use crate::{
    consts,
    log::{self, Format},
    paths,
};

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
        anyhow::bail!("link target does not exist: {}", from.display().quote());
    }

    let real_target = fs::canonicalize(from)
        .with_context(|| format!("failed to resolve real path of {}", from.display().quote()))?;

    if !real_target.is_file() {
        anyhow::bail!(
            "resolved link target is not a regular file: {}",
            real_target.display().quote()
        );
    }

    if let Some(parent) = to.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display().quote()))?;
    }

    // symlink_metadata does not follow links, so dangling symlinks are seen too.
    match fs::symlink_metadata(to) {
        Ok(meta) => {
            if meta.is_dir() {
                anyhow::bail!(
                    "cannot replace directory with a link: {}",
                    to.display().quote()
                );
            }

            fs::remove_file(to).with_context(|| {
                format!("failed to remove existing link at {}", to.display().quote())
            })?;
        }

        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => {
            return Err(e).with_context(|| format!("failed to inspect {}", to.display().quote()));
        }
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(&real_target, to).with_context(|| {
        format!(
            "failed to symlink {} -> {}",
            to.display().quote(),
            real_target.display().quote()
        )
    })?;

    #[cfg(windows)]
    packages::link::asset::write_shim(to, "", &[], &real_target, &[]).with_context(|| {
        format!(
            "failed to write shim {} -> {}",
            to.display().quote(),
            real_target.display().quote()
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
                .with_context(|| format!("Failed to chmod {}", path.display().quote()))?;
        }
    }

    #[cfg(windows)]
    {
        let mut perms = fs::metadata(path)?.permissions();

        if perms.readonly() {
            perms.set_readonly(false);
            fs::set_permissions(path, perms).with_context(|| {
                format!(
                    "Failed to clear read-only attribute on {}",
                    path.display().quote()
                )
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

// TODO: move this elsewhere, such as a web module?
pub fn perform_request(url: &str) -> anyhow::Result<(BodyReader<'static>, u64)> {
    if !url.starts_with("https://") {
        anyhow::bail!("This only performs 'https' requests. URL: {}.", url.quote());
    }

    let response = ureq::get(url)
        .header("User-Agent", consts::APP_NAME)
        .call()?;

    let len = response.body().content_length().unwrap_or(0);
    Ok((response.into_body().into_reader(), len))
}

pub fn download_file(url: &str, dest: &mut impl Write) -> anyhow::Result<()> {
    let (mut reader, total) = perform_request(url)?;
    let pb = ProgressBar::new(total);

    pb.set_style(log::get_progress_bar("Downloading")?);
    io::copy(&mut pb.wrap_read(&mut reader), dest)?;
    pb.finish_and_clear();

    Ok(())
}

pub fn extract_entry_from_zip(zip: impl Read + Seek, entry_name: &str) -> anyhow::Result<Vec<u8>> {
    let mut archive = ZipArchive::new(zip)?;
    let mut entry = archive
        .by_name(entry_name)
        .with_context(|| format!("Entry {} not found in zip", entry_name.quote()))?;

    let mut contents = Vec::with_capacity(entry.size() as usize);
    entry.read_to_end(&mut contents)?;

    Ok(contents)
}

#[cfg(unix)]
fn sudo_owner() -> Option<(u32, u32)> {
    if !nix::unistd::geteuid().is_root() {
        return None;
    }

    let uid: u32 = std::env::var("SUDO_UID").ok()?.parse().ok()?;
    let gid: u32 = std::env::var("SUDO_GID").ok()?.parse().ok()?;

    (uid != 0).then_some((uid, gid))
}

pub fn give_root_dir_to_user() -> anyhow::Result<()> {
    let root = &paths::root_dir();

    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;

        let Some((uid, gid)) = sudo_owner() else {
            // this isn't needed if we didn't run the command with sudo
            return Ok(());
        };

        fn walk(path: &Path, uid: u32, gid: u32) -> anyhow::Result<()> {
            let meta = fs::symlink_metadata(path)?;

            if meta.uid() != uid {
                std::os::unix::fs::lchown(path, Some(uid), Some(gid))
                    .with_context(|| format!("Failed to chown {}", path.display().quote()))?;
            }

            if meta.is_dir() {
                for entry in fs::read_dir(path)? {
                    walk(&entry?.path(), uid, gid)?;
                }
            }

            Ok(())
        }

        if root.exists() {
            walk(root, uid, gid)?;
        }
    }

    Ok(())
}
