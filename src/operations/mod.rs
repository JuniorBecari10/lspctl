use crate::{
    end, error, header,
    operations::util::{Action, OperationResult},
};

mod logic;
pub mod model;
mod prelude;
pub mod util;

pub fn install(args: model::InstallArgs) -> OperationResult {
    util::run_action(args.pkgs, args.yes, Action::Install, logic::install_pkg)
}

pub fn remove(args: model::RemoveArgs) -> OperationResult {
    util::run_action(args.pkgs, args.yes, Action::Remove, logic::remove_pkg)
}

pub fn list(args: model::ListArgs) -> OperationResult {
    // TODO: disable 'Registry is already downloaded' log here
    let (registry, _, state, _lock) = prelude::prelude();

    if args.installed {
        let keys = state.installed.keys().cloned().collect::<Vec<_>>();
        let (installed, missing) = util::filter_registry(registry, keys.as_slice());

        if !missing.is_empty() {
            for m in missing {
                error!("Package '{m}' doesn't exist.");
            }

            return OperationResult::Failure;
        }

        if installed.is_empty() {
            end!("There are no packages installed.");
            return OperationResult::Success;
        }

        header!("Installed packages:\n");
        util::write_entries(&installed, args.verbose);
    } else {
        header!("All packages:\n");
        util::write_entries(&registry.0, args.verbose);
    }

    OperationResult::Success
}

pub fn search(args: model::SearchArgs) -> OperationResult {
    todo!()
}
