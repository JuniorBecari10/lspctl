use crate::{
    packages,
    registry::{
        model::{Entry, Platform},
        resolver,
    },
    state::State,
};

// this mutates and writes state down
pub fn install_pkg(
    e: Entry,
    host: &Platform,
    state: &mut State,
    output: Option<&str>,
) -> anyhow::Result<()> {
    let entry = resolver::resolve_entry(e, host)?;
    packages::install(&entry, state, output)?;

    state.save()
}

// this function has some extra arguments for it to be accepted in 'run_action' despite not using them
pub fn remove_pkg(
    e: Entry,
    _host: &Platform,
    state: &mut State,
    _o: Option<&str>,
) -> anyhow::Result<()> {
    remove(&e.name, state)
}

pub fn remove(name: &str, state: &mut State) -> anyhow::Result<()> {
    packages::remove(name, state)?;
    state.save()
}
