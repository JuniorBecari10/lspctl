use std::{fs::File, io::Write};

use crate::consts;

pub fn download_file(url: &str, dest: &mut File) -> anyhow::Result<()> {
    let data = perform_request(url)?;
    dest.write_all(data.as_bytes())?;

    Ok(())
}

pub fn perform_request(url: &str) -> anyhow::Result<String> {
    Ok(ureq::get(url)
        .header("User-Agent", consts::APP_NAME)
        .call()?
        .body_mut()
        .read_to_string()?)
}
