mod cli;
mod consts;
mod folders;
mod io;
mod operations;
mod registry;
mod root;

fn main() {
    colog::init();
    cli::cli();
}
