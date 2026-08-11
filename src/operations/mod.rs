use crate::operations::util::{Action, OperationResult};

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
    let (registry, _, state) = prelude::prelude();

    dbg!(registry);
    dbg!(args);
    dbg!(state);

    OperationResult::Success
}
