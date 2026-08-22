use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use crate::registry::model::{Purl, ResolvedSource};

// just to not clone the name :)
fn url_source(purl: &Purl) -> Cow<'_, str> {
    match purl.namespace {
        Some(ref namespace) => Cow::Owned(format!("{namespace}/{}", purl.name)),
        None => Cow::Borrowed(&purl.name),
    }
}

/// select a file from Asset to use it here
pub fn github_url(source: &ResolvedSource, file: &str) -> String {
    format!(
        "https://github.com/{}/releases/download/{}/{}",
        url_source(&source.purl),
        source.purl.version,
        file,
    )
}

pub fn get_target(bin: &str, pkg_path: &Path) -> anyhow::Result<PathBuf> {
    match bin.split_once(':') {}
}
