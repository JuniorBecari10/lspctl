use anyhow::anyhow;

use crate::registry::model::Platform;

pub fn get_platform(s: &str) -> Option<Platform> {
    use Platform::*;

    match s {
        "unix" => Some(Unix),
        "darwin" => Some(Darwin),
        "linux" => Some(Linux),
        "win" => Some(Windows),
        _ => None,
    }
}

pub fn convert_platforms(platforms: Option<Vec<String>>) -> anyhow::Result<Option<Vec<Platform>>> {
    platforms
        .map(|v| {
            v.iter()
                .map(|p| get_platform(p).ok_or_else(|| anyhow!("Invalid platform: {}", p)))
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .transpose()
}
