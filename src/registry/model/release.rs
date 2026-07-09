use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Release {
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseAsset {
    pub name: String,

    #[serde(rename = "browser_download_url")]
    pub url: String,
}
