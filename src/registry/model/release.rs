use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReleaseAsset {
    pub name: String,

    #[serde(rename = "browser_download_url")]
    pub url: String,
}
