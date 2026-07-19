mod cli;
mod consts;
mod global;
mod io;
mod operations;
mod paths;
mod registry;
mod root;

fn main() {
    colog::init();
    cli::cli();
}
