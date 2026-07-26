use std::process::ExitCode;

mod cli;
mod consts;
mod global;
mod io;
mod logging;
mod operations;
mod paths;
mod registry;
mod root;

fn main() -> ExitCode {
    logging::init();
    cli::cli().into()
}
