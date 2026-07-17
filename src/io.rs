use std::{
    io::{ErrorKind, Write},
    path::Path,
};

use tempfile::NamedTempFile;

use crate::folders;

/// creates a new temporary file in lspctl/tmp.
/// requires tmp to exist. error if not.
/// it doesn't create it because it is expected to exist at this point.
fn new_temp() -> anyhow::Result<NamedTempFile> {
    Ok(NamedTempFile::new_in(folders::tmp_dir())?)
}

// this function must be atomic
// TODO: make persist_replace since some features need it
fn persist(temp: NamedTempFile, p: &Path) -> anyhow::Result<()> {
    match temp.persist_noclobber(p) {
        Ok(_) => Ok(()),
        Err(e) if e.error.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(e.error.into()),
    }
}

pub fn new_file_atomic(p: &Path) -> anyhow::Result<()> {
    let temp = new_temp()?;
    persist(temp, p)
}

pub fn new_file_atomic_write(p: &Path, contents: &[u8]) -> anyhow::Result<()> {
    let mut temp = new_temp()?;
    temp.write_all(contents)?;
    persist(temp, p)
}
