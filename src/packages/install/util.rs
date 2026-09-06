use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use indicatif::{ProgressBar, ProgressBarIter, ProgressStyle};

enum ArchiveKind {
    TarGz,
    TarXz,
    TarZstd,
    TarBz2,
    Gzip,
    Zip,
    Raw,
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
            anyhow::bail!("Archive has non-directory destination '{d}'")
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
    pb.set_style(
        ProgressStyle::with_template(
            "     Extracting [{bar:40.cyan/blue}] {bytes}/{total_bytes} {eta}",
        )?
        .progress_chars("=>-"),
    );

    Ok(pb.wrap_read(file))
}
