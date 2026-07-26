use crate::{registry, root};

mod logic;
pub mod model;

// TODO: when we add file locks, return it here as well in a tuple
/// Setups the root folder structure, reads the registry, parses it and hands over the result to the caller.
fn prelude() -> registry::model::Registry {
    root::setup_root().expect("Cannot create root folder structure");
    registry::read_registry().expect("Cannot read registry")
}

pub fn install(args: model::InstallArgs) {
    let registry = prelude();

    for pkg in args.pkgs {
        log::info!("Installing package '{pkg}'..");

        match logic::install_pkg(&pkg, &registry) {
            Ok(()) => log::info!("Package installed successfully."),
            Err(e) => log::error!("Failed to install '{pkg}': {e}."),
        }
    }
}

pub fn remove(args: model::RemoveArgs) {
    let registry = prelude();
    dbg!(args);
    dbg!(registry);
}

pub fn list(args: model::ListArgs) {
    dbg!(args);
}
