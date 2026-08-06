use std::fmt::Display;

use anyhow::anyhow;
use packageurl::PackageUrl;

use crate::registry::model::{
    Arch, AssetVars, Entry, InstallKind, Libc, OneOrMany, OneOrMap, Os, PackageManager, Platform,
    Purl, Variant,
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
// and perform some changes.
impl<'a> TryFrom<PackageUrl<'a>> for Purl {
    type Error = anyhow::Error;

    fn try_from(purl: PackageUrl<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: get_install_kind(purl.ty())
                .ok_or_else(|| anyhow!("Invalid install kind: '{}'", purl.ty()))?,

            namespace: purl.namespace().map(Into::into),
            name: sanitize_path_component(purl.name()),

            version: sanitize_path_component(
                &purl
                    .version()
                    .map(Into::<String>::into)
                    .ok_or_else(|| anyhow::anyhow!("Expected version number"))?,
            ),

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
            InstallKind::Go => Ok(PackageManager::Go),
            InstallKind::Cargo => Ok(PackageManager::Cargo),
            InstallKind::Gem => Ok(PackageManager::Gem),
            InstallKind::Composer => Ok(PackageManager::Composer),
            InstallKind::LuaRocks => Ok(PackageManager::LuaRocks),
            InstallKind::Opam => Ok(PackageManager::Opam),
            InstallKind::NuGet => Ok(PackageManager::NuGet),
            InstallKind::GitHub | InstallKind::Generic | InstallKind::OpenVSX => Err(
                anyhow::anyhow!("'{kind}' is not a package-manager install kind"),
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

    /// How many fields this constraint pins down. Used to prefer the
    /// more specific match if an asset array has overlapping targets.
    pub fn specificity(&self) -> u8 {
        1 + self.arch.is_some() as u8 + self.libc.is_some() as u8
    }
}

// yes, the space is intended
fn display_option<T: Display>(opt: Option<T>) -> String {
    opt.map_or_else(String::new, |t| format!(" {t}"))
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.os,
            display_option(self.arch),
            display_option(self.libc)
        )
    }
}

impl Display for Os {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Os::Linux => write!(f, "Linux"),
            Os::Darwin => write!(f, "Darwin"),
            Os::Windows => write!(f, "Windows"),
        }
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arch::X64 => write!(f, "x64"),
            Arch::X86 => write!(f, "x86"),
            Arch::Arm64 => write!(f, "Arm64"),
            Arch::Arm => write!(f, "Arm"),
            Arch::Armv6l => write!(f, "Arm-v6L"),
            Arch::Armv7l => write!(f, "Arm-v7L"),
        }
    }
}
impl Display for Libc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Libc::Gnu => write!(f, "GLibc"),
            Libc::Musl => write!(f, "Musl"),
            Libc::OpenBSD => write!(f, "OpenBSD"),
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} ({})",
            self.name, self.source.purl.version, self.source.purl.kind
        )
    }
}

impl Display for InstallKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallKind::Npm => write!(f, "npm"),
            InstallKind::PyPI => write!(f, "PyPI"),
            InstallKind::Go => write!(f, "Go"),
            InstallKind::Cargo => write!(f, "Cargo"),
            InstallKind::Gem => write!(f, "Gem"),
            InstallKind::Composer => write!(f, "Composer"),
            InstallKind::LuaRocks => write!(f, "LuaRocks"),
            InstallKind::Opam => write!(f, "Opam"),
            InstallKind::NuGet => write!(f, "NuGet"),
            InstallKind::GitHub => write!(f, "GitHub"),
            InstallKind::Generic => write!(f, "Generic"),
            InstallKind::OpenVSX => write!(f, "OpenVSX"),
        }
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::PackageManager {
                manager: _,
                extra_packages: _,
            } => write!(f, "package manager"),
            Variant::Asset(_) => write!(f, "asset"),
            Variant::Download(_) => write!(f, "download"),
            Variant::Build(_) => write!(f, "build"),
        }
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

fn sanitize_path_component(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());

    for c in raw.trim().chars() {
        match c {
            '/' | '\\' => out.push('_'),
            '<' | '>' | ':' | '"' | '|' | '?' | '*' => out.push('_'),

            c if (c as u32) < 0x20 => {}
            c => out.push(c),
        }
    }

    while out.ends_with('.') || out.ends_with(' ') {
        out.pop();
    }

    if out.is_empty() || out == "." || out == ".." {
        out = "unknown".to_string();
    }

    // Windows reserved device names are invalid regardless of extension
    const RESERVED: &[&str] = &[
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    let stem = out.split('.').next().unwrap_or(&out);
    if RESERVED.iter().any(|r| stem.eq_ignore_ascii_case(r)) {
        out = format!("_{out}");
    }

    if out.len() > 200 {
        out.truncate(200);
    }

    out
}
