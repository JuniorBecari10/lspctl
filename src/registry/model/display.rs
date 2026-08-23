use std::fmt::Display;

use crate::registry::model::{Arch, Entry, InstallKind, Libc, Os, Platform, Variant};
use colored::Colorize;

impl Entry {
    pub fn format_line(&self, installed: bool) -> String {
        let line = format!(
            "{} {} ({})",
            self.name, self.source.purl.version, self.source.purl.kind
        );

        let line = if self.deprecation.is_some() {
            format!("{} (deprecated)", line.strikethrough())
        } else {
            line
        };

        if installed {
            format!("{line} {}", "(installed)".green())
        } else {
            line
        }
    }

    pub fn print_line(&self, name_width: usize, version_width: usize, installed: bool) {
        let name = if self.deprecation.is_some() {
            self.name.bold().strikethrough().dimmed()
        } else {
            self.name.bold()
        };

        let marker = if installed {
            format!("  {}", "(installed)".green())
        } else {
            String::new()
        };

        println!(
            "{}{:width$}  {:<version_width$}  {}{marker}",
            name,
            "",
            self.source.purl.version.cyan(),
            self.source.purl.kind.to_string().dimmed(),
            width = name_width.saturating_sub(self.name.len()),
            version_width = version_width,
        );
    }

    pub fn print_detailed(&self, installed: bool) {
        if self.deprecation.is_some() {
            println!("{}", self.name.bold().strikethrough());
        } else {
            println!("{}", self.name.bold());
        }

        println!(
            "{}",
            "─"
                .repeat(
                    self.description
                        .find('\n')
                        .unwrap_or(self.description.len())
                )
                .dimmed()
        );

        println!("{}", self.description);
        println!();

        if installed {
            println!("  {}", "● Installed".green().bold());
            println!();
        }

        println!("  {:<12} {}", "Homepage:", self.homepage.blue().underline());
        println!("  {:<12} {}", "Version:", self.source.purl.version);
        println!("  {:<12} {}", "Source:", self.source.purl.kind);
        println!("  {:<12} {}", "Licenses:", self.licenses.join(", "));
        println!("  {:<12} {}", "Languages:", self.languages.join(", "));
        println!("  {:<12} {}", "Categories:", self.categories.join(", "));

        if let Some(dep) = &self.deprecation {
            println!();
            println!(
                "  {} Deprecated since {}: {}",
                "[!]".yellow().bold(),
                dep.since,
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
