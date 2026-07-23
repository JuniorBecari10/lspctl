use anyhow::anyhow;
use packageurl::PackageUrl;

use crate::registry::model::{AssetVars, InstallKind, OneOrMany, OneOrMap, PackageManager, Purl};

impl<T> From<OneOrMany<T>> for Vec<T> {
    fn from(value: OneOrMany<T>) -> Self {
        match value {
            OneOrMany::One(t) => vec![t],
            OneOrMany::Many(t) => t,
        }
    }
}

// just convert all Cow fields into owned ones,
// and 'ty' into an enum
impl<'a> TryFrom<PackageUrl<'a>> for Purl {
    type Error = anyhow::Error;

    fn try_from(purl: PackageUrl<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: get_install_kind(purl.ty())
                .ok_or(anyhow!("Invalid install kind: '{}'", purl.ty()))?,
            namespace: purl.namespace().map(Into::into),
            name: purl.name().into(),
            version: purl.version().map(Into::into),
            qualifiers: purl
                .qualifiers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            subpath: purl.subpath().map(Into::into),
        })
    }
}

// ooooh boilerplate
impl TryFrom<InstallKind> for PackageManager {
    type Error = anyhow::Error;

    fn try_from(kind: InstallKind) -> Result<Self, Self::Error> {
        match kind {
            InstallKind::Npm => Ok(PackageManager::Npm),
            InstallKind::PyPI => Ok(PackageManager::PyPI),
            InstallKind::Golang => Ok(PackageManager::Golang),
            InstallKind::Cargo => Ok(PackageManager::Cargo),
            InstallKind::Gem => Ok(PackageManager::Gem),
            InstallKind::Composer => Ok(PackageManager::Composer),
            InstallKind::LuaRocks => Ok(PackageManager::LuaRocks),
            InstallKind::Opam => Ok(PackageManager::Opam),
            InstallKind::NuGet => Ok(PackageManager::NuGet),
            InstallKind::GitHub | InstallKind::Generic | InstallKind::OpenVsx => Err(
                anyhow::anyhow!("{kind:?} is not a package-manager install kind"),
            ),
        }
    }
}

impl OneOrMap {
    pub fn map(self, f: impl Fn(String) -> String) -> Self {
        match self {
            OneOrMap::One(s) => OneOrMap::One(f(s)),
            OneOrMap::Map(m) => OneOrMap::Map(m.into_iter().map(|(k, v)| (f(k), f(v))).collect()),
        }
    }
}

impl AssetVars {
    pub fn map(self, f: impl Fn(String) -> String) -> Self {
        match self {
            AssetVars::Path(p) => AssetVars::Path(f(p)),
            AssetVars::Nested(n) => {
                AssetVars::Nested(n.into_iter().map(|(k, v)| (f(k), f(v))).collect())
            }
        }
    }
}

fn get_install_kind(s: &str) -> Option<InstallKind> {
    use InstallKind::*;

    match s {
        "github" => Some(GitHub),
        "npm" => Some(Npm),
        "pypi" => Some(PyPI),
        "golang" => Some(Golang),
        "cargo" => Some(Cargo),
        "gem" => Some(Gem),
        "generic" => Some(Generic),
        "openvsx" => Some(OpenVsx),
        "composer" => Some(Composer),
        "luarocks" => Some(LuaRocks),
        "opam" => Some(Opam),
        "nuget" => Some(NuGet),
        _ => None,
    }
}
