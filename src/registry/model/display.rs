use std::fmt::Display;

use crate::{
    log::Format,
    registry::model::{Arch, Category, Entry, InstallKind, Libc, Os, Platform, Variant},
    table::{fields::Fields, util},
};

use colored::Colorize;

impl Entry {
    pub fn print_detailed(&self, installed_version: Option<String>, show_marker: bool) {
        let title = if self.deprecation.is_some() {
            self.name.bold().strikethrough()
        } else {
            self.name.bold()
        };
        let first_line = self.description.lines().next().unwrap_or_default();

        println!("{title}");
        println!("{}", util::rule(util::display_width(first_line)));
        println!("{}", self.description);
        println!();

        if installed_version.is_some() && show_marker {
            println!("  {}", "Installed".green().bold());
            println!();
        }

        let registry_version = self.source.purl.version.as_str();

        let mut fields = Fields::new();
        fields.field("Homepage", self.homepage.url().to_string());

        match installed_version.as_deref() {
            Some(installed) if installed != registry_version => {
                fields
                    .field("Registry Version", registry_version)
                    .field("Installed Version", installed);
            }
            Some(_) => {
                fields.field(
                    "Version",
                    format!("{registry_version}  {}", "(matches registry)".green()),
                );
            }
            None => {
                fields.field("Version", registry_version);
            }
        }

        fields
            .field("Source", self.source.purl.kind.to_string())
            .list("Licenses", &self.licenses, "No licenses")
            .list("Languages", &self.languages, "No languages")
            .list("Categories", &self.categories, "No categories")
            .list("Bins", self.bin.iter().flat_map(|b| b.keys()), "No bins");

        fields.print();

        if let Some(dep) = &self.deprecation {
            println!();
            println!(
                "  {} Deprecated since {}: {}",
                "[!]".yellow().bold(),
                dep.since.cyan(),
                dep.message
            );
        }

        println!();
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

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Compiler => write!(f, "Compiler"),
            Category::DAP => write!(f, "DAP"),
            Category::Formatter => write!(f, "Formatter"),
            Category::LSP => write!(f, "LSP"),
            Category::Linter => write!(f, "Linter"),
            Category::Runtime => write!(f, "Runtime"),
        }
    }
}
