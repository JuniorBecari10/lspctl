use std::process::ExitCode;

mod cli;
mod consts;
mod disk;
mod global;
mod log;
mod operations;
mod packages;
mod paths;
mod registry;
mod setup;
mod state;
mod table;

fn main() -> ExitCode {
    let result = cli::cli();

    if let Err(e) = disk::give_root_dir_to_user() {
        error!("Error while trying to give root directory to user: {}", e);
        return ExitCode::FAILURE;
    }

    result.into()
}
