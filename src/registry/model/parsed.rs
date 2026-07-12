// TODO: remove this as soon as they are actually used
#![allow(unused)]

use std::{collections::HashMap, fs::OpenOptions};

use packageurl::PackageUrl;

// TODO: cache this once parsed (JSON serialized structs) and invalidate it when updating.
// if the cache file is missing, parse again.
// OneOrMany becomes a vec of one element if the variant is One

// TODO: resolve strings like {{ version | strip_prefix "v" }}

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
    purl: Purl,
    variant: SourceVariant,
    supported_platforms: Option<Vec<Platform>>,
    bin: Option<String>, // for js-debug-adapter (edge case)
}

#[derive(Debug)]
pub enum SourceVariant {
    PackageManager {
        // value from purl.kind but restrained to package managers
        manager: PackageManager,
        extra_packages: Option<Vec<String>>,
    },

    Asset(Vec<Asset>),
    Download(Option<Downloads>),
    Build(Vec<Build>),
}

// implements TryFrom<PackageUrl>
#[derive(Debug)]
pub struct Purl {
    pub kind: InstallKind,
    pub namespace: Option<String>,
    pub name: String,
    pub version: Option<String>,
    pub qualifiers: HashMap<String, String>,
    pub subpath: Option<String>,
}

#[derive(Debug)]
pub enum InstallKind {
    Npm,
    Pypi,
    Golang,
    Cargo,
    Gem,
    Composer,
    LuaRocks,
    Opam,
    Nuget,
    GitHub,
    Generic,
    OpenVsx,
}

#[derive(Debug)]
pub enum PackageManager {
    Npm,
    Pypi,
    Golang,
    Cargo,
    Gem,
    Composer,
    LuaRocks,
    Opam,
    Nuget,
}

#[derive(Debug)]
pub enum Platform {
    Unix,
    Darwin,
    Linux,
    Windows,
}

#[derive(Debug)]
pub struct Asset {
    targets: Option<Vec<String>>,
    files: Vec<String>,
    // bin
}

#[derive(Debug)]
pub enum Downloads {
    Simple { file: String },
    Detailed(Vec<Download>),
}

#[derive(Debug)]
pub struct Download {
    targets: Option<Vec<String>>,
    files: Vec<String>,
    bin: Option<String>,
}

#[derive(Debug)]
pub struct Build {
    command: String,
    target: Option<Vec<String>>,
    // bin
    env: Option<HashMap<String, String>>,
}
