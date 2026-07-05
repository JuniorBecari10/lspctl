use anyhow::anyhow;

mod model;
mod util;

const MASON_URL: &str = "https://api.github.com/repos/mason-org/mason-registry/releases/latest";

fn get_latest_release() -> anyhow::Result<()> {
    log::info!("Fetching registry...");

    let data = parse_release(&util::perform_request(MASON_URL)?)?;
    let asset = find_registry_asset(&data)?;

    let mut temp_zip = tempfile::tempfile()?;
    util::download_file(&asset.browser_download_url, &mut temp_zip)?;

    Ok(())
}

fn find_registry_asset(release: &model::Release) -> anyhow::Result<&model::Asset> {
    release
        .assets
        .iter()
        .find(|a| a.name == "registry")
        .ok_or_else(|| anyhow!("registry.json.zip not found in release assets"))
}

fn parse_release(raw_json: &str) -> anyhow::Result<model::Release> {
    Ok(serde_json::from_str(raw_json)?)
}

fn download_registry() {}
