use std::{
    path::Path,
    process::{Command, Stdio},
};

use anyhow::Context;

use crate::{note, registry::model::PackageManager};

pub fn get_install_command(manager: PackageManager, name: &str, version: &str) -> String {}

pub fn run_command(
    binary: &str,
    args: &[&str],
    folder: &Path,
    env: &[(&str, &str)],
) -> anyhow::Result<()> {
    let command = format!("{binary} {}", args.join(" "));
    note!("Running: {command}");

    let mut cmd = Command::new(binary);
    cmd.args(args)
        .current_dir(folder)
        .envs(env.iter().copied())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd
        .status()
        .with_context(|| format!("Failed to launch '{binary}'. Is it on PATH?"))?;

    if !status.success() {
        anyhow::bail!(
            "'{command}' exited with exit code {}",
            status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "Unknown status".into())
        );
    }

    Ok(())
}
