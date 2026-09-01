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
    #[serde(rename = "file", with = "one_or_many")]
    pub files: Vec<String>,
    pub bin: Option<OneOrMap>,
    #[serde(flatten)]
    pub variables: HashMap<String, AssetVars>, // ad-hoc fields like "lsp" / "dap" / "man"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Downloads {
    Simple { file: String },
    Detailed(Vec<Download>), // TODO: if necessary, write 'with = "one_or_many"' here
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
    #[serde(flatten)]
    pub extra: HashMap<String, String>, // erlang_ls / els_dap
}

mod one_or_many {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(items: &[T], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        match items {
            [one] => one.serialize(serializer),
            many => many.serialize(serializer),
        }
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Repr<T> {
        One(T),
        Many(Vec<T>),
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        Ok(match Repr::<T>::deserialize(deserializer)? {
            Repr::One(t) => vec![t],
            Repr::Many(v) => v,
        })
    }
}
