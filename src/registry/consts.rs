use const_format::{concatcp, formatcp};

use crate::def_consts;

const ITEMS_PER_PAGE: u32 = 15;

def_consts!(
    RELEASES_API_URL = "https://api.github.com/repos/mason-org/mason-registry/releases",
    REGISTRY_FILE = "registry.json",
    REGISTRY_ZIP = concatcp!(REGISTRY_FILE, ".zip"),
    RELEASE_PAGE_URL = formatcp!("{RELEASES_API_URL}?per_page={ITEMS_PER_PAGE}&page="),
);
