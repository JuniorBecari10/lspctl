use std::fs::{File, OpenOptions, TryLockError};

use crate::{
    consts, error, fatal, global,
    log::{Fatal, Format},
    note, paths,
    registry::{
        self,
        model::{Platform, Registry},
    },
    root,
    state::State,
    step,
};

pub struct ProcessLock {
    _file: File,
}

type Prelude = (Registry, Platform, State, ProcessLock);

// do NOT ignore the lock file. bind it to something like '_lock'
// for it to exist throughout the entire function
pub fn prelude() -> Prelude {
    let lock = acquire_lock();
    let maybe_tag = setup_root();
    let mut state = load_state();

    if let Some(tag) = maybe_tag {
        state.set_registry_tag(tag);
        state.save().fatal("Couldn't save state");
    }

    root::clean_orphans(&state);
    (read_registry(), get_platform(), state, lock)
}

// ---

/// returns the tag of the newly created registry, if any
fn setup_root() -> Option<String> {
    root::setup_root().fatal("Cannot create root folder structure")
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

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .fatal(&format!("Failed to create directory {}", parent.display()));
    }

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&path)
        .fatal(&format!("Failed to open lock file at {}", path.display()));

    match file.try_lock() {
        Ok(()) => {}

        Err(TryLockError::WouldBlock) => {
            step!(
                "One instance of {} is already running. Waiting for the lock to be released..",
                consts::APP_NAME
            );

            note!(
                "If this is an error, you can run {}",
                format!("{} delete lockfile", consts::APP_NAME).quote()
            );

            note!("to delete the lockfile if no other instances are running.");
            file.lock().fatal("Failed to acquire process lock");
        }

        Err(TryLockError::Error(e)) => {
            fatal!("Failed to acquire process lock: {e}");
        }
    }

    ProcessLock { _file: file }
}
