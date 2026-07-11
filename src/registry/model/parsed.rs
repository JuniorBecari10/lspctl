use std::collections::HashMap;

// OneOrMany becomes a vec of one element if the variant is One

#[derive(Debug)]
pub struct Registry(Vec<Entry>);

#[derive(Debug)]
pub struct Entry {
    name: String,
    description: String,
    homepage: String,
    licenses: Vec<String>,
    languages: Vec<String>,
    categories: Vec<String>,
    source: Source,
    bin: Option<HashMap<String, String>>, // change to Vec<String> of entries?
}

#[derive(Debug)]
pub struct Source {
    id: String, // parse purl
    variant: SourceVariant,
}

#[derive(Debug)]
pub enum SourceVariant {
    PackageManager {
        // package_manager
        extra_packages: Option<Vec<String>>,
    },

    Asset {
        assets: Vec<Asset>,
    },

    Download {},

    Build {},
}

#[derive(Debug)]
pub struct Asset {
    targets: Vec<String>,
}
