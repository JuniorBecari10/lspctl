use std::{
    fmt::Display,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::Context;

use crate::{note, registry::model::PackageManager};

pub struct InstallCommand {
    binary: String,
    args: Vec<String>,
}

impl Display for InstallCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.binary, self.args.join(" "))
    }
}

pub fn get_install_command(manager: PackageManager, name: &str, version: &str) -> InstallCommand {}

pub fn run_command(
    command: InstallCommand,
    folder: &Path,
    env: &[(&str, &str)],
) -> anyhow::Result<()> {
    let command_str = command.to_string();
    note!("Running: {command_str}");

    let mut cmd = Command::new(command.binary.clone());
    cmd.args(command.args)
        .current_dir(folder)
        .envs(env.iter().copied())
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
