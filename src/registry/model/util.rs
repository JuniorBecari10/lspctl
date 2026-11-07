use anyhow::anyhow;
use packageurl::PackageUrl;

use crate::{
    log::Format,
    registry::model::{
        AssetVars, InstallKind, OneOrMany, OneOrMap, PackageManager, Platform, Purl,
    },
};

const MAX_COMPONENT_LEN: usize = 200;

// Characters that are invalid in Windows file names (besides '/').
const WINDOWS_INVALID: &[char] = &['\\', '<', '>', ':', '"', '|', '?', '*'];

// Windows device names are invalid as a file stem, with or without an extension.
const WINDOWS_RESERVED: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

impl<T> From<OneOrMany<T>> for Vec<T> {
    fn from(value: OneOrMany<T>) -> Self {
        match value {
            OneOrMany::One(t) => vec![t],
            OneOrMany::Many(t) => t,
        }
    }
}

// just convert all Cow fields into owned ones,
// and perform some changes.
impl<'a> TryFrom<PackageUrl<'a>> for Purl {
    type Error = anyhow::Error;

    fn try_from(purl: PackageUrl<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: get_install_kind(purl.ty())
                .ok_or_else(|| anyhow!("Invalid install kind: {}", purl.ty().quote()))?,

            namespace: purl
                .namespace()
                .map(Into::into)
                .map(sanitize_path_component),

            name: sanitize_path_component(purl.name()),

            version: purl
                .version()
                .map(sanitize_path_component)
                .ok_or_else(|| anyhow::anyhow!("Expected version number"))?,

            qualifiers: purl
                .qualifiers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),

            subpath: purl.subpath().map(sanitize_path_component),
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
            InstallKind::Go => Ok(PackageManager::Go),
            InstallKind::Cargo => Ok(PackageManager::Cargo),
            InstallKind::Gem => Ok(PackageManager::Gem),
            InstallKind::Composer => Ok(PackageManager::Composer),
            InstallKind::LuaRocks => Ok(PackageManager::LuaRocks),
            InstallKind::Opam => Ok(PackageManager::Opam),
            InstallKind::NuGet => Ok(PackageManager::NuGet),
            InstallKind::GitHub | InstallKind::Generic | InstallKind::OpenVSX => {
                Err(anyhow::anyhow!(
                    "{} is not a package-manager install kind",
                    kind.to_string().quote()
                ))
            }
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

    /// how many fields this constraint satisfies. the more fields,
    /// the more specific, and therefore preferred.
    pub fn specificity(&self) -> u8 {
        1 + self.arch.is_some() as u8 + self.libc.is_some() as u8
    }
}

impl PackageManager {
    pub fn get_command(&self) -> String {
        match self {
            PackageManager::Npm => "npm".into(),
            PackageManager::PyPI => "python".into(),
            PackageManager::Go => "go".into(),
            PackageManager::Cargo => "cargo".into(),
            PackageManager::Gem => "gem".into(),
            PackageManager::Composer => "composer".into(),
            PackageManager::LuaRocks => "luarocks".into(),
            PackageManager::Opam => "opam".into(),
            PackageManager::NuGet => "dotnet".into(),
        }
    }
}

impl Purl {
    pub fn qualified_package_name(&self) -> String {
        match &self.namespace {
            Some(ns) => format!("{ns}/{}", self.name),
            None => self.name.clone(),
        }
    }
}

fn get_install_kind(s: &str) -> Option<InstallKind> {
    use InstallKind::*;

    match s {
        "github" => Some(GitHub),
        "npm" => Some(Npm),
        "pypi" => Some(PyPI),
        "golang" => Some(Go),
        "cargo" => Some(Cargo),
        "gem" => Some(Gem),
        "generic" => Some(Generic),
        "openvsx" => Some(OpenVSX),
        "composer" => Some(Composer),
        "luarocks" => Some(LuaRocks),
        "opam" => Some(Opam),
        "nuget" => Some(NuGet),
        _ => None,
    }
}

// TODO: apply the Windows restriction to all OSs,
// since the user may want to sync between the two filesystems?
/// this sanitizes a string to be valid as a file/directory name on your target OS.
/// this has conditional code that rejects more names in Windows, but '/' is blocked everywhere.
fn sanitize_path_component(raw: &str) -> String {
    let mut out: String = raw
        .chars()
        .filter(|c| !c.is_control())
        .map(|c| match c {
            '/' => '_',
            c if cfg!(windows) && WINDOWS_INVALID.contains(&c) => '_',
            c => c,
        })
        .collect();

    // Truncate on a char boundary (String::truncate panics otherwise).
    // Done before trimming so the cut can't leave a trailing '.' or ' '.
    if out.len() > MAX_COMPONENT_LEN {
        let mut end = MAX_COMPONENT_LEN;
        while !out.is_char_boundary(end) {
            end -= 1;
        }
        out.truncate(end);
    }

    // Windows silently strips trailing dots/spaces, so "a." and "a" would alias.
    let s = if cfg!(windows) {
        out.trim_start()
            .trim_end_matches(|c: char| c == '.' || c.is_whitespace())
    } else {
        out.as_str()
    };

    // "." and ".." only vanish via the Windows trim above, so they must be
    // checked explicitly everywhere else.
    if s.is_empty() || s == "." || s == ".." {
        return "unknown".to_owned();
    }

    if cfg!(windows) {
        // "CON .txt" is also reserved, hence the trim_end on the stem.
        let stem = s.split('.').next().unwrap_or(s).trim_end();
        if WINDOWS_RESERVED
            .iter()
            .any(|r| stem.eq_ignore_ascii_case(r))
        {
            return format!("_{s}");
        }
    }

    s.to_owned()
}
