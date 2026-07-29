use crate::{
    global,
    log::Fatal,
    registry::{
        self,
        model::{Platform, Registry},
    },
    root,
    state::State,
};

pub fn prelude() -> (Registry, Platform, State) {
    setup_root();
    (read_registry(), get_platform(), load_state())
}

// ---

fn setup_root() {
    root::setup_root().fatal("Cannot create root folder structure");
}

fn read_registry() -> Registry {
    registry::read_registry().fatal("Cannot read registry")
}

fn get_platform() -> Platform {
    global::current_platform().fatal("Cannot get current platform")
}

fn load_state() -> State {
    State::load().fatal("Cannot read state")
}
