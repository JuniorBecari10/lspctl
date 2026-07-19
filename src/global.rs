#[cfg(target_env = "gnu")]
use crate::registry::model::Libc;
use crate::registry::model::{Arch, Os, Platform};

pub fn current_platform() -> Platform {
    let os = match std::env::consts::OS {
        "linux" => Os::Linux,
        "macos" => Os::Darwin,
        "windows" => Os::Windows,
        other => panic!("unsupported OS: {other}"),
    };

    let arch = match std::env::consts::ARCH {
        "x86_64" => Some(Arch::X64),
        "x86" => Some(Arch::X86),
        "aarch64" => Some(Arch::Arm64),
        "arm" => Some(Arch::Arm),
        _ => None,
    };

    Platform {
        os,
        arch,
        libc: platform_libc(os),
    }
}

#[cfg(target_env = "musl")]
fn platform_libc(os: Os) -> Option<Libc> {
    matches!(os, Os::Linux).then_some(Libc::Musl)
}

#[cfg(target_env = "gnu")]
fn platform_libc(os: Os) -> Option<Libc> {
    matches!(os, Os::Linux).then_some(Libc::Gnu)
}

#[cfg(not(any(target_env = "musl", target_env = "gnu")))]
fn platform_libc(_os: Os) -> Option<Libc> {
    None // e.g. darwin/windows have no relevant libc value in this registry's scheme
}
