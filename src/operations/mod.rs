use crate::{
    error, global, note,
    operations::util::{OperationResult, accepted_installation},
    registry::{
        self,
        model::{Platform, Registry},
    },
    root, step,
};

mod logic;
pub mod model;
pub mod util;

fn prelude() -> (Registry, Platform) {
    root::setup_root().expect("Cannot create root folder structure");

    (
        registry::read_registry().expect("Cannot read registry"),
        global::current_platform().expect("Cannot get current platform"),
    )
}

pub fn install(args: model::InstallArgs) -> OperationResult {
    let (registry, platform) = prelude();
    let (entries, missing) = util::filter_registry(registry, &args.pkgs);

    if !missing.is_empty() {
        for m in missing {
            error!("Package '{m}' doesn't exist.");
        }

        return OperationResult::Failure;
    }

    if !accepted_installation(&entries, args.yes) {
        return OperationResult::Success;
    }

    for pkg in entries {
        let name = pkg.name.clone();
        step!("Installing package '{name}'..");

        match logic::install_pkg(pkg, &platform) {
            Ok(()) => note!("Package installed successfully."),
            Err(e) => error!("Failed to install '{}': {e}.", name),
        }
    }

    OperationResult::Success
}

pub fn remove(args: model::RemoveArgs) -> OperationResult {
    let registry = prelude();
    dbg!(args);
    dbg!(registry);

    OperationResult::Success
}

pub fn list(args: model::ListArgs) -> OperationResult {
    dbg!(args);
    OperationResult::Success
}
