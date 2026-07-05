mod cli;
mod consts;
mod operations;
mod registry;
mod root;

fn main() {
    env_logger::init();
    cli::cli();
}
