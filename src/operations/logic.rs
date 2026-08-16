use crate::{
    packages,
    registry::{
        model::{Entry, Platform, ResolvedEntry},
        resolver,
    },
    state::State,
};

// this mutates and writes state down
fn resolve_and_perform(
    e: Entry,
    host: &Platform,
    state: &mut State,
    op: fn(&ResolvedEntry, &mut State) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let entry = resolver::resolve_entry(e, host)?;
    op(&entry, state)?;

    state.save()
}

pub fn install_pkg(e: Entry, host: &Platform, state: &mut State) -> anyhow::Result<()> {
    resolve_and_perform(e, host, state, packages::install)
}

pub fn remove_pkg(e: Entry, host: &Platform, state: &mut State) -> anyhow::Result<()> {
    resolve_and_perform(e, host, state, packages::remove)
}
