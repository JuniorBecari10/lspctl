use crate::{
    packages,
    registry::{
        model::{Entry, Platform},
        resolver,
    },
    state::State,
};

// this mutates and writes state down
pub fn install_pkg(e: Entry, platform: &Platform, state: &mut State) -> anyhow::Result<()> {
    let entry = resolver::resolve_entry(e, platform)?;

    packages::install(entry, state)?;
    state.save()
}
