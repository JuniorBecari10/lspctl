use std::fmt::Display;

use crate::registry::model::{Arch, Entry, InstallKind, Libc, Os, Platform, Variant};
use colored::Colorize;

impl Entry {
    pub fn print_detailed(&self, installed_version: Option<String>) {
        const SHORT_WIDTH: usize = 12;
        const LONG_WIDTH: usize = 19;

        let needs_two_versions = installed_version
            .as_ref()
            .is_some_and(|ver| *ver != self.source.purl.version);

        let label_width = if needs_two_versions {
            LONG_WIDTH
        } else {
            SHORT_WIDTH
        };

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

        if installed_version.is_some() {
            println!("  {}", "Installed".green().bold());
            println!();
        }

        println!(
            "  {:<label_width$} {}",
            "Homepage:",
            self.homepage.blue().underline()
        );

        match installed_version {
            Some(ver) if needs_two_versions => {
                println!(
                    "  {:<label_width$} {}",
                    "Registry Version:", self.source.purl.version
                );

                println!("  {:<label_width$} {}", "Installed Version:", ver);
            }

            Some(_) => println!(
                "  {:<label_width$} {}  {}",
                "Version:",
                self.source.purl.version,
                "(matches registry)".green()
            ),

            None => println!(
                "  {:<label_width$} {}",
                "Version:", self.source.purl.version
            ),
        };

        println!("  {:<label_width$} {}", "Source:", self.source.purl.kind);

        println!(
            "  {:<label_width$} {}",
            "Licenses:",
            self.licenses.join(", ")
        );

        println!(
            "  {:<label_width$} {}",
            "Languages:",
            self.languages.join(", ")
        );

        println!(
            "  {:<label_width$} {}",
            "Categories:",
            self.categories.join(", ")
        );

        if let Some(bins) = &self.bin {
            println!(
                "  {:<label_width$} {}",
                "Bins:",
                bins.keys()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

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
