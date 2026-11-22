use std::path::PathBuf;

// this is to still get your user's folder even if running in sudo
pub fn data_local_dir() -> Option<PathBuf> {
    #[cfg(unix)]
    {
        use nix::unistd::{User, geteuid};

        // Only trust SUDO_USER when we're actually root, in case it's a stale env var.
        if geteuid().is_root()
            && let Some(name) = std::env::var_os("SUDO_USER")
            && let Ok(Some(user)) = User::from_name(&name.to_string_lossy())
        {
            return Some(user.dir.join(".local").join("share"));
        }
    }

    dirs::data_local_dir()
}
