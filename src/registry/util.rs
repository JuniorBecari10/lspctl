use std::{fs::File, io::Write, path::Path};

async fn download_file(url: &str, dest: &Path) -> anyhow::Result<()> {
    let data = reqwest::get(url).await?.text().await?;
    let mut file = File::create(dest)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
