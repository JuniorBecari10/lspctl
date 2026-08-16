use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

enum ArchiveKind {
    TarGz,
    Zip, // also covers .vsix, .jar, .phar, .artifactbundle.zip. same format under the hood
    Raw, // no extension, or .exe. not an archive, place as-is
    Unsupported(&'static str),
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
            let file = File::open(downloaded)?; // fresh handle. don't rely on the download's cursor position
            let gz = flate2::read::GzDecoder::new(file);

            tar::Archive::new(gz).unpack(&target_dir)?;
            Ok(())
        }

        ArchiveKind::Zip => {
            let target_dir = resolve_target_dir(dest, tmp_pkg_path)?;
            let file = File::open(downloaded)?;

            zip::ZipArchive::new(file)?.extract(&target_dir)?;
            Ok(())
        }

        ArchiveKind::Raw => {
            let final_path = match dest {
                Some(d) if d.ends_with('/') => {
                    let dir = tmp_pkg_path.join(d);
                    fs::create_dir_all(&dir)?;
                    dir.join(source_name)
                }

                Some(d) => tmp_pkg_path.join(d), // rename, e.g. ktlint -> ktlint.jar
                None => tmp_pkg_path.join(source_name),
            };

            fs::rename(downloaded, &final_path)?;
            make_executable(&final_path)?;
            Ok(())
        }

        ArchiveKind::Unsupported(what) => {
            anyhow::bail!("'{source_name}' needs {what}, which is not supported")
        }
    }
}

fn resolve_target_dir(dest: Option<&str>, tmp_pkg_path: &Path) -> anyhow::Result<PathBuf> {
    match dest {
        Some(d) if d.ends_with('/') => {
            let dir = tmp_pkg_path.join(d);
            fs::create_dir_all(&dir)?;
            Ok(dir)
        }

        Some(d) => anyhow::bail!("Archive has non-directory destination '{d}'"),
        None => Ok(tmp_pkg_path.to_path_buf()),
    }
}

fn detect_archive_kind(filename: &str) -> ArchiveKind {
    let lower = filename.to_ascii_lowercase();

    if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") {
        ArchiveKind::TarGz
    } else if lower.ends_with(".zip")
        || lower.ends_with(".vsix")
        || lower.ends_with(".jar")
        || lower.ends_with(".phar")
        || lower.ends_with(".artifactbundle.zip")
    {
        ArchiveKind::Zip
    } else if lower.ends_with(".gz") {
        // bare gzip, single file, NOT a tar archive
        ArchiveKind::Unsupported("bare .gz decompression")
    } else if lower.ends_with(".tar.xz") {
        ArchiveKind::Unsupported(".tar.xz decompression")
    } else if lower.ends_with(".tar.zst") {
        ArchiveKind::Unsupported(".tar.zst decompression")
    } else if lower.ends_with(".tar.bz2") {
        ArchiveKind::Unsupported(".tar.bz2 decompression")
    } else {
        // no extension, or .exe. a raw binary, not an archive
        ArchiveKind::Raw
    }
}

#[cfg(unix)]
fn make_executable(path: &Path) -> anyhow::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = fs::metadata(path)?.permissions();
    let mode = perms.mode();

    perms.set_mode(mode | 0o111); // add execute for owner/group/other, preserve rest
    fs::set_permissions(path, perms)?;
    Ok(())
}

#[cfg(windows)]
fn make_executable(_path: &Path) -> anyhow::Result<()> {
    Ok(()) // no execute-bit concept on Windows filesystems
}
