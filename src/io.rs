use std::{
    io::{ErrorKind, Write},
    path::Path,
};

use anyhow::anyhow;
use tempfile::NamedTempFile;

fn new_temp_in(p: &Path) -> anyhow::Result<NamedTempFile> {
    Ok(NamedTempFile::new_in(p.parent().ok_or_else(|| {
        anyhow!("Path {:?} does not have a parent directory", p)
    })?)?)
}

fn persist(temp: NamedTempFile, p: &Path) -> anyhow::Result<()> {
    match temp.persist_noclobber(p) {
        Ok(_) => Ok(()),
        Err(e) if e.error.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(e.error.into()),
    }
}

pub fn new_file_atomic(p: &Path) -> anyhow::Result<()> {
    let temp = new_temp_in(p)?;
    persist(temp, p)?;
    Ok(())
}

pub fn new_file_atomic_write(p: &Path, contents: &[u8]) -> anyhow::Result<()> {
    let mut temp = new_temp_in(p)?;
    temp.write_all(contents)?;
    persist(temp, p)?;
    Ok(())
}
