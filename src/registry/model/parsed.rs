use serde::{Deserialize, Serialize};

use crate::registry::model::{AssetVars, Deprecation, OneOrMap};
use std::collections::HashMap;

// All fields like this has templates:
// - bin
// - file(s)
// - every hashmap
// - env

#[derive(Debug, Serialize)]
pub struct Registry(pub Vec<Entry>);

#[derive(Debug, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
pub struct Source {
    pub purl: Purl,
    pub variant: Variant,
    pub supported_platforms: Vec<Platform>,
    pub version_overrides: Option<Vec<VersionOverride>>,
    pub bin: Option<String>, // for js-debug-adapter (edge case)
}

// implements TryFrom<PackageUrl>
#[derive(Debug, Clone, Serialize)]
pub struct Purl {
    pub kind: InstallKind,
    pub namespace: Option<String>,
    pub name: String,
    pub version: String,
    pub qualifiers: HashMap<String, String>,
    pub subpath: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Variant {
    PackageManager {
        manager: PackageManager,
        extra_packages: Vec<String>,
    },
    Asset(Vec<Asset>),
    Download(Downloads),
    Build(Vec<Build>),
}

#[derive(Debug, Clone, Serialize)]
pub struct VersionOverride {
    pub constraint: String,
    pub id: String,
    pub variant: Variant,
    pub supported_platforms: Vec<Platform>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallKind {
    Npm,
    PyPI,
    Go,
    Cargo,
    Gem,
    Composer,
    LuaRocks,
    Opam,
    NuGet,
    GitHub,
    Generic,
    OpenVSX,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageManager {
    Npm,
    PyPI,
    Go,
    Cargo,
    Gem,
    Composer,
    LuaRocks,
    Opam,
    NuGet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Os {
    Linux,
    Darwin,
    Windows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Arch {
    X64,
    X86,
    Arm64,
    Arm,
    Armv6l,
    Armv7l,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Libc {
    Gnu,
    Musl,
    OpenBSD,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Platform {
    pub os: Os,
    pub arch: Option<Arch>,
    pub libc: Option<Libc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub targets: Vec<Platform>,
    pub files: Vec<String>,
    pub bin: Option<OneOrMap>,
    pub variables: HashMap<String, AssetVars>, // ad-hoc fields like "lsp" / "dap" / "man"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Downloads {
    Simple { file: String },
    Detailed(Vec<Download>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub targets: Vec<Platform>,
    pub files: HashMap<String, String>,
    pub bin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub command: String,
    pub targets: Vec<Platform>,
    pub bin: Option<OneOrMap>,
    pub env: Option<HashMap<String, String>>,
    pub staged: Option<bool>,
    pub extra: HashMap<String, String>, // erlang_ls / els_dap
}
