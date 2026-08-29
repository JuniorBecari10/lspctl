use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    path::Path,
    process::{Command, Stdio},
};

use maplit::hashmap;

use anyhow::Context;

use crate::{note, paths, registry::model::PackageManager};

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
    note!("Running: '{command_str}'..");

    let mut cmd = Command::new(command.binary.clone());
    cmd.args(command.args)
        .current_dir(dir)
        .envs(command.env)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd
        .status()
        .with_context(|| format!("Failed to launch '{}'. Is it on PATH?", command.binary))?;

    if !status.success() {
        anyhow::bail!(
            "'{command_str}' exited with exit code {}",
            status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "<unknown>".into())
        );
    }

    note!("Command executed successfully.");
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

// This MUST be atomic.
pub fn move_package(name: &str) -> anyhow::Result<()> {
    let from = paths::tmp_dir().join(name);
    let to = paths::packages_dir().join(name);

    fs::rename(from, to)?;
    Ok(())
}
