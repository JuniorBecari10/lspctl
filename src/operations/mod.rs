use crate::{
    error, note,
    operations::util::{OperationResult, accepted_installation},
    step,
};

mod logic;
pub mod model;
mod prelude;
pub mod util;

pub fn install(args: model::InstallArgs) -> OperationResult {
    let (registry, platform, mut state) = prelude::prelude();
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

        match logic::install_pkg(pkg, &platform, &mut state) {
            Ok(()) => note!("Package installed successfully."),
            Err(e) => error!("Failed to install '{}': {e}.", name),
        }
    }

    OperationResult::Success
}

pub fn remove(args: model::RemoveArgs) -> OperationResult {
    let (registry, platform, state) = prelude::prelude();

    dbg!(args);
    dbg!(registry);
    dbg!(platform);

    OperationResult::Success
}

pub fn list(args: model::ListArgs) -> OperationResult {
    let (registry, _, state) = prelude::prelude();

    dbg!(registry);
    dbg!(args);
    dbg!(state);

    OperationResult::Success
}
