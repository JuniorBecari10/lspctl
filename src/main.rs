use std::process::ExitCode;

mod cli;
mod consts;
mod global;
mod io;
mod log;
mod operations;
mod paths;
mod registry;
mod root;

fn main() -> ExitCode {
    cli::cli().into()
}
