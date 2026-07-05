use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Release {
    pub assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}
