use std::process::ExitCode;

mod cli;
mod consts;
mod global;
mod io;
mod log;
mod operations;
mod packages;
mod paths;
mod registry;
mod root;
mod state;

fn main() -> ExitCode {
    cli::cli().into()
}
