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
mod root;
mod state;

fn main() -> ExitCode {
    cli::cli().into()
}
