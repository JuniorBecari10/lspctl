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

    // TODO: if the operation fails, call automatically the repair command?
    // it's better to be half-installed when the package is there and the state isn't,
    // because we won't have it pointing to not-installed packages,
    // and also that we can't enforce atomicity in both write operations at once.
    state.save()
}
