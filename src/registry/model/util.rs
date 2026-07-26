use anyhow::anyhow;
use packageurl::PackageUrl;

use crate::registry::model::{
    AssetVars, InstallKind, OneOrMany, OneOrMap, PackageManager, Platform, Purl,
};

impl<T> From<OneOrMany<T>> for Vec<T> {
    fn from(value: OneOrMany<T>) -> Self {
        match value {
            OneOrMany::One(t) => vec![t],
            OneOrMany::Many(t) => t,
        }
    }
}

// just convert all Cow fields into owned ones,
// 'ty' into an enum
// and 'version' into a non-option
impl<'a> TryFrom<PackageUrl<'a>> for Purl {
    type Error = anyhow::Error;

    fn try_from(purl: PackageUrl<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: get_install_kind(purl.ty())
                .ok_or_else(|| anyhow!("Invalid install kind: '{}'", purl.ty()))?,

            namespace: purl.namespace().map(Into::into),
            name: purl.name().into(),

            version: purl
                .version()
                .map(Into::<String>::into)
                .ok_or_else(|| anyhow::anyhow!("Expected version number"))?,

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
    pub fn try_map(self, f: impl Fn(String) -> anyhow::Result<String>) -> anyhow::Result<Self> {
        Ok(match self {
            OneOrMap::One(s) => OneOrMap::One(f(s)?),
            OneOrMap::Map(m) => OneOrMap::Map(
                m.into_iter()
                    .map(|(k, v)| Ok((k, f(v)?)))
                    .collect::<anyhow::Result<_>>()?,
            ),
        })
    }
}

impl AssetVars {
    pub fn try_map(self, f: impl Fn(String) -> anyhow::Result<String>) -> anyhow::Result<Self> {
        Ok(match self {
            AssetVars::Path(s) => AssetVars::Path(f(s)?),
            AssetVars::Nested(m) => AssetVars::Nested(
                m.into_iter()
                    .map(|(k, v)| Ok((k, f(v)?)))
                    .collect::<anyhow::Result<_>>()?,
            ),
        })
    }
}

impl Platform {
    // may be partially specified: 'arch'/'libc' of 'None' mean "matches any";
    // 'host' is the concrete platform being installed for.
    pub fn matches(&self, host: &Platform) -> bool {
        if self.os != host.os {
            return false;
        }

        if let Some(arch) = self.arch
            && Some(arch) != host.arch
        {
            return false;
        }

        if let Some(libc) = self.libc
            && Some(libc) != host.libc
        {
            return false;
        }

        true
    }

    /// How many fields this constraint pins down. used to prefer the
    /// more specific match if an asset array has overlapping targets.
    pub fn specificity(&self) -> u8 {
        1 + self.arch.is_some() as u8 + self.libc.is_some() as u8
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
