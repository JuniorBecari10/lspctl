use crate::root;

pub mod model;

fn prelude() {
    root::setup_root().expect("Cannot create root folder structure");
}

pub fn install(args: model::InstallArgs) {
    prelude();
    dbg!(args);
}

pub fn remove(args: model::RemoveArgs) {
    prelude();
    dbg!(args);
}

pub fn list(args: model::ListArgs) {
    dbg!(args);
}
