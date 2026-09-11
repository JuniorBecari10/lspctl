use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::Context;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressBarIter, ProgressStyle};
use maplit::hashmap;

use crate::{
    disk,
    log::{self, Format},
    note, paths,
    registry::model::{PackageManager, Purl},
};

enum ArchiveKind {
    TarGz,
    TarXz,
    TarZstd,
    TarBz2,
    Gzip,
    Zip,
    Raw,
}

pub struct InstallCommand {
    binary: String,
    args: Vec<String>,
    env: HashMap<String, String>,
}

impl Display for InstallCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.binary, self.args.join(" "))
    }
}

pub fn parse_file_spec(spec: &str) -> (&str, Option<&str>) {
    match spec.split_once(':') {
        Some((source, dest)) => (source, Some(dest)),
        None => (spec, None),
    }
}

pub fn place_or_extract(
    downloaded: &Path,
    source_name: &str,
    dest: Option<&str>,
    tmp_pkg_path: &Path,
) -> anyhow::Result<()> {
    match detect_archive_kind(source_name) {
        ArchiveKind::TarGz => {
            let target_dir = resolve_target_dir(dest, tmp_pkg_path)?;
            let gz = flate2::read::GzDecoder::new(wrapped_file(downloaded)?);

            tar::Archive::new(gz).unpack(target_dir)?;
            Ok(())
        }

        ArchiveKind::TarXz => {
            let target_dir = resolve_target_dir(dest, tmp_pkg_path)?;
            let xz = xz2::read::XzDecoder::new(wrapped_file(downloaded)?);

            tar::Archive::new(xz).unpack(target_dir)?;
            Ok(())
        }

        ArchiveKind::TarZstd => {
            let target_dir = resolve_target_dir(dest, tmp_pkg_path)?;
            let zstd = zstd::stream::read::Decoder::new(wrapped_file(downloaded)?)?;

            tar::Archive::new(zstd).unpack(target_dir)?;
            Ok(())
        }

        ArchiveKind::TarBz2 => {
            let target_dir = resolve_target_dir(dest, tmp_pkg_path)?;
            let bz2 = bzip2::read::BzDecoder::new(wrapped_file(downloaded)?);

            tar::Archive::new(bz2).unpack(target_dir)?;
            Ok(())
        }

        ArchiveKind::Zip => {
            let target_dir = resolve_target_dir(dest, tmp_pkg_path)?;
            zip::ZipArchive::new(wrapped_file(downloaded)?)?.extract(target_dir)?;

            Ok(())
        }

        ArchiveKind::Gzip => extract_gzip(downloaded, source_name, dest, tmp_pkg_path),

        ArchiveKind::Raw => {
            let target = resolve_file_destination(dest, source_name, tmp_pkg_path);

            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::rename(downloaded, &target)?;
            make_executable(&target)?;

            Ok(())
        }
    }
}

fn extract_gzip(
    downloaded: &Path,
    source_name: &str,
    dest: Option<&str>,
    tmp_pkg_path: &Path,
) -> anyhow::Result<()> {
    let target = match dest {
        Some(d) if d.ends_with('/') => {
            let dir = tmp_pkg_path.join(d);
            fs::create_dir_all(&dir)?;

            let filename = source_name
                .strip_suffix(".gz")
                .or_else(|| source_name.strip_suffix(".GZ"))
                .unwrap_or(source_name);

            dir.join(filename)
        }

        Some(d) => {
            let target = tmp_pkg_path.join(d);

            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }

            target
        }

        None => {
            let filename = source_name
                .strip_suffix(".gz")
                .or_else(|| source_name.strip_suffix(".GZ"))
                .unwrap_or(source_name);

            tmp_pkg_path.join(filename)
        }
    };

    let mut decoder = flate2::read::GzDecoder::new(wrapped_file(downloaded)?);

    let mut output = File::create(&target)?;
    io::copy(&mut decoder, &mut output)?;

    output.sync_all()?;
    make_executable(&target)?;

    Ok(())
}

fn resolve_target_dir(dest: Option<&str>, tmp_pkg_path: &Path) -> anyhow::Result<PathBuf> {
    match dest {
        Some(d) if d.ends_with('/') => {
            let dir = tmp_pkg_path.join(d);
            fs::create_dir_all(&dir)?;
            Ok(dir)
        }

        Some(d) => {
            anyhow::bail!("Archive has non-directory destination: {}", d.quote())
        }

        None => Ok(tmp_pkg_path.to_path_buf()),
    }
}

fn resolve_file_destination(dest: Option<&str>, source_name: &str, tmp_pkg_path: &Path) -> PathBuf {
    match dest {
        Some(d) if d.ends_with('/') => tmp_pkg_path.join(d).join(source_name),
        Some(d) => tmp_pkg_path.join(d),
        None => tmp_pkg_path.join(source_name),
    }
}

fn detect_archive_kind(filename: &str) -> ArchiveKind {
    let lower = filename.to_ascii_lowercase();

    if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") {
        ArchiveKind::TarGz
    } else if lower.ends_with(".tar.xz") || lower.ends_with(".txz") {
        ArchiveKind::TarXz
    } else if lower.ends_with(".tar.zst") || lower.ends_with(".tzst") {
        ArchiveKind::TarZstd
    } else if lower.ends_with(".tar.bz2") || lower.ends_with(".tbz2") {
        ArchiveKind::TarBz2
    } else if lower.ends_with(".zip")
        || lower.ends_with(".vsix")
        || lower.ends_with(".jar")
        || lower.ends_with(".phar")
    {
        ArchiveKind::Zip
    } else if lower.ends_with(".gz") {
        ArchiveKind::Gzip
    } else {
        ArchiveKind::Raw
    }
}

#[cfg(unix)]
fn make_executable(path: &Path) -> anyhow::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(perms.mode() | 0o111);
    fs::set_permissions(path, perms)?;

    Ok(())
}

#[cfg(windows)]
fn make_executable(_path: &Path) -> anyhow::Result<()> {
    Ok(())
}

// ---

fn wrapped_file(path: &Path) -> anyhow::Result<ProgressBarIter<File>> {
    let file = File::open(path)?;
    let len = file.metadata()?.len();

    let pb = ProgressBar::new(len);
    pb.set_style(log::get_progress_bar("Extracting")?);

    Ok(pb.wrap_read(file))
}

pub fn get_install_commands(
    manager: PackageManager,
    name: &str,
    version: &str,
    extra_packages: &[String],
    tmp_pkg_path: &Path,
) -> Vec<InstallCommand> {
    match manager {
        PackageManager::PyPI => {
            let venv_pip = if cfg!(windows) {
                tmp_pkg_path.join("Scripts").join("pip.exe")
            } else {
                tmp_pkg_path.join("bin").join("pip")
            };

            vec![
                InstallCommand {
                    binary: "python3".into(),
                    args: vec!["-m".into(), "venv".into(), ".".into()],
                    env: hashmap! {},
                },
                InstallCommand {
                    binary: venv_pip.to_string_lossy().into_owned(),
                    args: vec!["install".into(), format!("{name}=={version}")],
                    env: hashmap! {},
                },
            ]
        }

        _ => {
            let binary = manager.get_command();
            let args = get_install_args(manager, name, version, extra_packages, tmp_pkg_path);
            let env = get_install_env(manager, tmp_pkg_path);

            vec![InstallCommand { binary, args, env }]
        }
    }
}

pub fn run_command(command: InstallCommand, dir: &Path) -> anyhow::Result<()> {
    let command_str = command.to_string();

    note!("{} {}", "Running".verb(), command_str.quote());

    let mut cmd = Command::new(command.binary.clone());
    cmd.args(command.args)
        .current_dir(dir)
        .envs(command.env)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd.status().with_context(|| {
        format!(
            "Failed to launch {}. Is it on PATH?",
            command.binary.quote()
        )
    })?;

    if !status.success() {
        anyhow::bail!(
            "{} exited with exit code {}",
            command_str.quote(),
            status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "<unknown>".into())
                .red()
        );
    }

    Ok(())
}

// ---

fn get_install_args(
    manager: PackageManager,
    name: &str,
    version: &str,
    extra_packages: &[String],
    pkg_dir: &Path,
) -> Vec<String> {
    match manager {
        PackageManager::Npm => vec![
            "install".into(),
            "--prefix".into(),
            ".".into(),
            format!("{name}@{version}"),
        ]
        .into_iter()
        .chain(extra_packages.iter().cloned())
        .collect(),

        PackageManager::Cargo => vec![
            "install".into(),
            "--root".into(),
            pkg_dir.to_string_lossy().into_owned(),
            name.into(),
        ],

        PackageManager::Gem => vec![
            "install".into(),
            "--no-user-install".into(),
            "--install-dir".into(),
            ".".into(),
            "--no-format-executable".into(),
            name.into(),
            "--version".into(),
            version.into(),
        ],

        PackageManager::Go => vec!["install".into(), format!("{name}@{version}")],

        PackageManager::LuaRocks => vec![
            "install".into(),
            "--tree".into(),
            ".".into(),
            name.into(),
            version.into(),
        ],

        PackageManager::NuGet => vec![
            "tool".into(),
            "install".into(),
            "--tool-path".into(),
            ".".into(),
            name.into(),
            "--version".into(),
            version.into(),
        ],

        // handled elsewhere
        PackageManager::PyPI => unreachable!(),

        PackageManager::Composer => todo!(),
        PackageManager::Opam => todo!(),
    }
}

fn get_install_env(manager: PackageManager, pkg_dir: &Path) -> HashMap<String, String> {
    match manager {
        PackageManager::Npm
        | PackageManager::Cargo
        | PackageManager::Gem
        | PackageManager::LuaRocks
        | PackageManager::NuGet => hashmap! {},

        PackageManager::Go => hashmap! {
            "GOBIN".to_string() => pkg_dir.join("bin").to_string_lossy().into_owned(),
            "GOMODCACHE".to_string() => pkg_dir.join("gomodcache").to_string_lossy().into_owned(),
        },

        // handled elsewhere
        PackageManager::PyPI => unreachable!(),

        PackageManager::Composer => todo!(),
        PackageManager::Opam => todo!(),
    }
}

// ---

fn openvsx_url(file: &str, purl: &Purl, namespace: &str) -> String {
    format!(
        "https://open-vsx.org/api/{namespace}/{}/{}/file/{file}",
        purl.name, purl.version
    )
}

pub fn install_openvsx(file: &str, purl: &Purl, tmp_pkg_path: &Path) -> anyhow::Result<()> {
    let namespace = purl
        .namespace
        .as_deref()
        .context("OpenVSX purl missing namespace")?;

    let url = openvsx_url(file, purl, namespace);
    download_and_place(&url, file, tmp_pkg_path)
}

pub fn download_and_place(url: &str, local_name: &str, tmp_pkg_path: &Path) -> anyhow::Result<()> {
    note!("URL: {}", url.quote());

    let scratch = tmp_pkg_path.join(format!(".download-{local_name}"));
    let mut f = File::create(&scratch)?;

    disk::download_file(url, &mut f)?;
    drop(f);

    place_or_extract(&scratch, local_name, None, tmp_pkg_path)
}

// ---

// This MUST be atomic.
pub fn move_package(name: &str) -> anyhow::Result<()> {
    let from = paths::tmp_dir().join(name);
    let to = paths::packages_dir().join(name);

    fs::rename(from, to)?;
    Ok(())
}
