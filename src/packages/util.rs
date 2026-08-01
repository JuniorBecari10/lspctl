use std::{
    collections::HashMap,
    fmt::Display,
    path::Path,
    process::{Command, Stdio},
};

use maplit::hashmap;

use anyhow::Context;

use crate::{note, registry::model::PackageManager};

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

pub fn get_install_command(
    manager: PackageManager,
    name: &str,
    version: &str,
    extra_packages: &[String],
    tmp_pkg_path: &Path,
) -> InstallCommand {
    let binary = manager.get_command();
    let args = get_install_args(manager, name, version, extra_packages);
    let env = get_install_env(manager, tmp_pkg_path);

    InstallCommand { binary, args, env }
}

pub fn run_command(command: InstallCommand, folder: &Path) -> anyhow::Result<()> {
    let command_str = command.to_string();
    note!("Running: '{command_str}'..");

    let mut cmd = Command::new(command.binary.clone());
    cmd.args(command.args)
        .current_dir(folder)
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
                .unwrap_or_else(|| "Unknown status".into())
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

        PackageManager::PyPI => todo!(),
        PackageManager::Cargo => todo!(),
        PackageManager::Gem => todo!(),
        PackageManager::Golang => vec!["install".into(), format!("{name}@{version}")],
        PackageManager::Composer => todo!(),
        PackageManager::LuaRocks => todo!(),
        PackageManager::Opam => todo!(),
        PackageManager::NuGet => todo!(),
    }
}

fn get_install_env(manager: PackageManager, pkg_dir: &Path) -> HashMap<String, String> {
    match manager {
        PackageManager::Npm => hashmap! {},
        PackageManager::PyPI => todo!(),

        PackageManager::Golang => hashmap! {
            "GOBIN".to_string() => pkg_dir.join("bin").to_string_lossy().into_owned(),
            "GOMODCACHE".to_string() => pkg_dir.join("gomodcache").to_string_lossy().into_owned(),
        },

        PackageManager::Cargo => todo!(),
        PackageManager::Gem => todo!(),
        PackageManager::Composer => todo!(),
        PackageManager::LuaRocks => todo!(),
        PackageManager::Opam => todo!(),
        PackageManager::NuGet => todo!(),
    }
}
