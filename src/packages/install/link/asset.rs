use crate::registry::model::{Asset, Source};

fn github_url(source: &Source, asset: &Asset) -> String {
    format!(
        "https://github.com/{}/releases/download/{}/{}",
        source.purl.name,
        source.purl.version,
        todo!("file")
    )
}
