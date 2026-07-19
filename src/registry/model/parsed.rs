#![allow(unused)]

use crate::registry::model::{AssetExtra, Deprecation, OneOrMap};
use std::collections::HashMap;

// TODO: cache this once parsed (JSON serialized structs) and invalidate it when updating.
// if the cache file is missing, parse again.
// TODO: resolve strings like {{ version | strip_prefix "v" }}

#[derive(Debug)]
pub struct Registry(pub Vec<Entry>);

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub licenses: Vec<String>,
    pub languages: Vec<String>,
    pub categories: Vec<String>,
    pub source: Source,
    pub bin: Option<HashMap<String, String>>,
    pub deprecation: Option<Deprecation>,
}

#[derive(Debug)]
pub struct Source {
    pub purl: Purl,
    pub variant: SourceVariant,
    pub supported_platforms: Vec<Platform>,
    pub version_overrides: Option<Vec<VersionOverride>>,
    pub bin: Option<String>, // for js-debug-adapter (edge case)
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
pub enum SourceVariant {
    PackageManager {
        manager: PackageManager,
        extra_packages: Vec<String>,
    },
    Asset(Vec<Asset>),
    Download(Downloads),
    Build(Vec<Build>),
}

#[derive(Debug)]
pub struct VersionOverride {
    pub constraint: String,
    pub id: String,
    pub variant: SourceVariant,
    pub supported_platforms: Vec<Platform>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InstallKind {
    Npm,
    Pypi,
    Golang,
    Cargo,
    Gem,
    Composer,
    LuaRocks,
    Opam,
    NuGet,
    GitHub,
    Generic,
    OpenVsx,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PackageManager {
    Npm,
    Pypi,
    Golang,
    Cargo,
    Gem,
    Composer,
    LuaRocks,
    Opam,
    NuGet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Os {
    Linux,
    Darwin,
    Windows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    X64,
    X86,
    Arm64,
    Arm,
    Armv6l,
    Armv7l,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Libc {
    Gnu,
    Musl,
    OpenBsd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Platform {
    pub os: Os,
    pub arch: Option<Arch>,
    pub libc: Option<Libc>,
}

#[derive(Debug)]
pub struct Asset {
    pub targets: Vec<Platform>,
    pub files: Vec<String>,
    pub bin: Option<OneOrMap>,
    pub extra: HashMap<String, AssetExtra>, // ad-hoc fields like "lsp" / "dap"
}

#[derive(Debug)]
pub enum Downloads {
    Simple { file: String },
    Detailed(Vec<Download>),
}

#[derive(Debug)]
pub struct Download {
    pub targets: Vec<Platform>,
    pub files: HashMap<String, String>,
    pub bin: Option<String>,
}

#[derive(Debug)]
pub struct Build {
    pub command: String,
    pub targets: Vec<Platform>,
    pub bin: Option<OneOrMap>,
    pub env: Option<HashMap<String, String>>,
    pub staged: Option<bool>,
    pub extra: HashMap<String, String>, // erlang_ls / els_dap
}
