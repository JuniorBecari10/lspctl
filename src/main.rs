use std::process::ExitCode;

mod cli;
mod consts;
mod global;
mod io;
mod operations;
mod paths;
mod registry;
mod root;

fn main() -> ExitCode {
    colog::init();
    cli::cli().into()
}
