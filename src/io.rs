use std::{
    io::{ErrorKind, Write},
    path::Path,
};

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
