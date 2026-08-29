use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use crate::{
    disk,
    registry::model::{Purl, ResolvedSource},
};

// just to not clone the name :)
fn url_source(purl: &Purl) -> Cow<'_, str> {
    match purl.namespace {
        Some(ref namespace) => Cow::Owned(format!("{namespace}/{}", purl.name)),
        None => Cow::Borrowed(&purl.name),
    }
}

/// select a file from Asset to use it here
pub fn github_url(source: &ResolvedSource, file: &str) -> String {
    format!(
        "https://github.com/{}/releases/download/{}/{}",
        url_source(&source.purl),
        source.purl.version,
        file,
    )
}

fn get_wrapper(wrapper: &str) -> Option<(&str, &[&str])> {
    match wrapper {
        "node" => Some((wrapper, &[])),
        "php" => Some((wrapper, &[])),
        "ruby" => Some((wrapper, &[])),
        "python" => Some(("python3", &[])),
        "java-jar" => Some(("java", &["-jar"])),
        "dotnet" => Some((wrapper, &[])),
        _ => None,
    }
}

pub fn get_target(name: &str, value: &str, bin: &Path, pkg_path: &Path) -> anyhow::Result<PathBuf> {
    match value.split_once(':') {
        // no scheme, value points to a runnable binary already,
        // so we only need to link it in the 'bin' folder.
        None => {
            let target = pkg_path.join(value);
            if !target.exists() {
                anyhow::bail!(
                    "Expected binary '{name}' at '{}' but it doesn't exist after extraction",
                    target.display()
                );
            }

            disk::link_files(&target, bin)?;
            Ok(bin.into())
        }

        Some((wrapper, path)) => {
            let target = pkg_path.join(path);
            let (interpreter, args) = get_wrapper(wrapper).ok_or_else(|| {
                anyhow::anyhow!("Unsupported wrapper '{wrapper}' for '{name}' in Asset entry")
            })?;

            write_shim(bin, interpreter, args, &target, &[])
        }
    }
}

pub fn write_shim(
    output: &Path,
    interpreter: &str,
    extra_args: &[&str],
    target: &Path,
    env: &[(&str, &str)],
) -> anyhow::Result<PathBuf> {
    if !target.exists() {
        anyhow::bail!("Shim target doesn't exist: '{}'", target.display());
    }

    #[cfg(unix)]
    {
        use std::{fs, os::unix::fs::PermissionsExt};
        let args_str = extra_args.join(" ");

        let env_str = env
            .iter()
            .map(|(key, value)| format!("{key}=\"{value}\""))
            .collect::<Vec<_>>()
            .join(" ");

        let script = if args_str.is_empty() {
            format!(
                "#!/bin/sh\n\
                {env_str} exec {interpreter} \"{}\" \"$@\"\n",
                target.display()
            )
        } else {
            format!(
                "#!/bin/sh\n\
                {env_str} exec {interpreter} {args_str} \"{}\" \"$@\"\n",
                target.display()
            )
        };

        fs::write(output, script)?;

        let mut perms = fs::metadata(output)?.permissions();
        perms.set_mode(perms.mode() | 0o111);
        fs::set_permissions(output, perms)?;
    }
    #[cfg(windows)]
    {
        use std::fs;

        let cmd_path = output.with_extension("cmd");
        let args_str = extra_args.join(" ");

        let env_str = env
            .iter()
            .map(|(key, value)| format!("set \"{key}={value}\"\r\n"))
            .collect::<String>();

        let script = format!(
            "@echo off\r\n\
            {env_str} {interpreter} {args_str} \"{}\" %*\r\n",
            target.display()
        );

        fs::write(&cmd_path, script)?;
    }

    Ok(output.to_path_buf())
}
