use std::fs::{File, OpenOptions, TryLockError};

use crate::{
    consts, end, error, fatal, global,
    log::Fatal,
    paths,
    registry::{
        self,
        model::{Platform, Registry},
    },
    root,
    state::State,
};

pub struct ProcessLock {
    _file: File,
}

type Prelude = (Registry, Platform, State, ProcessLock);

pub fn prelude() -> Prelude {
    prelude_inner(true)
}

pub fn prelude_no_log() -> Prelude {
    prelude_inner(false)
}

// do NOT ignore the lock file. bind it to something like '_lock'
// for it to exist throughout the entire function
fn prelude_inner(log: bool) -> Prelude {
    setup_root(log);

    (
        read_registry(),
        get_platform(),
        load_state(),
        acquire_lock(),
    )
}

// ---

fn setup_root(log: bool) {
    root::setup_root(log).fatal("Cannot create root folder structure");
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

// ---

pub fn acquire_lock() -> ProcessLock {
    let path = paths::lock_file();

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&path)
        .fatal(&format!("Failed to open lock file at {}", path.display()));

    match file.try_lock() {
        Ok(()) => {}

        Err(TryLockError::WouldBlock) => {
            end!(
                "One instance of {} is already running. Waiting for the lock to be released..",
                consts::APP_NAME
            );

            file.lock().fatal("Failed to acquire process lock");
        }

        Err(TryLockError::Error(e)) => {
            fatal!("Failed to acquire process lock: {e}");
        }
    }

    ProcessLock { _file: file }
}
