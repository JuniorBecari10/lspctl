use crate::{registry, root};

pub mod model;

// TODO: load registry here and pass to the functions
fn prelude() {
    // TODO: skip this if the files (registry) already exists
    root::setup_root().expect("Cannot create root folder structure");
}

pub fn install(args: model::InstallArgs) {
    prelude();
    dbg!(args);

    let registry = registry::read_registry().expect("Cannot read registry");
    dbg!(registry);
}

pub fn remove(args: model::RemoveArgs) {
    prelude();
    dbg!(args);
}

pub fn list(args: model::ListArgs) {
    dbg!(args);
}
