use crate::registry::model::{Arch, Libc, Os, Platform};

pub fn get_platform(s: &str) -> anyhow::Result<Vec<Platform>> {
    if s == "unix" {
        return Ok(vec![
            Platform {
                os: Os::Linux,
                arch: None,
                libc: None,
            },
            Platform {
                os: Os::Darwin,
                arch: None,
                libc: None,
            },
        ]);
    }

    let parts: Vec<&str> = s.split('_').collect();

    let os = match parts[0] {
        "darwin" => Os::Darwin,
        "linux" => Os::Linux,
        "win" => Os::Windows,
        other => anyhow::bail!("Unknown OS '{other}' in target '{s}'"),
    };

    if parts.len() == 1 {
        return Ok(vec![Platform {
            os,
            arch: None,
            libc: None,
        }]);
    }

    let arch = match parts[1] {
        "x64" => Arch::X64,
        "x86" => Arch::X86,
        "arm64" => Arch::Arm64,
        "arm" => Arch::Arm,
        "armv6l" => Arch::Armv6l,
        "armv7l" | "armv7" => Arch::Armv7l,
        other => anyhow::bail!("unknown arch '{other}' in target '{s}'"),
    };

    let libc = match parts.get(2) {
        None => None,
        Some(&"gnu") => Some(Libc::Gnu),
        Some(&"musl") => Some(Libc::Musl),
        Some(&"openbsd") => Some(Libc::OpenBSD),
        Some(other) => anyhow::bail!("unknown libc/variant '{other}' in target '{s}'"),
    };

    Ok(vec![Platform {
        os,
        arch: Some(arch),
        libc,
    }])
}

pub fn convert_platforms(platforms: Option<Vec<String>>) -> anyhow::Result<Vec<Platform>> {
    let Some(platforms) = platforms else {
        return Ok(Vec::new()); // no restriction specified, matches everything
    };

    platforms
        .iter()
        .map(|s| get_platform(s))
        .collect::<anyhow::Result<Vec<Vec<Platform>>>>()
        .map(|nested| nested.into_iter().flatten().collect())
}
