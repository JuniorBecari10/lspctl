use crate::{
    operations::util,
    packages,
    registry::model::{Entry, Platform},
    state::State,
};

// this mutates and writes state down
pub fn install_pkg(e: Entry, platform: &Platform, state: &mut State) -> anyhow::Result<()> {
    let (entry, asset) = util::resolve_entry(e, platform)?;

    packages::install(entry, state, asset)?;
    state.save()
}
