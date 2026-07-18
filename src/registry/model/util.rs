use anyhow::anyhow;
use packageurl::PackageUrl;

use crate::registry::model::{InstallKind, OneOrMany, Purl};

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

fn get_install_kind(s: &str) -> Option<InstallKind> {
    use InstallKind::*;

    match s {
        "github" => Some(GitHub),
        "npm" => Some(Npm),
        "pypi" => Some(Pypi),
        "golang" => Some(Golang),
        "cargo" => Some(Cargo),
        "gem" => Some(Gem),
        "generic" => Some(Generic),
        "openvsx" => Some(OpenVsx),
        "composer" => Some(Composer),
        "luarocks" => Some(LuaRocks),
        "opam" => Some(Opam),
        "nuget" => Some(Nuget),
        _ => None,
    }
}
