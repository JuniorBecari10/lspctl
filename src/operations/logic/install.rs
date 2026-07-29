use crate::{
    operations::util,
    registry::model::{Entry, Platform},
    state::State,
};

pub fn install_pkg(e: Entry, platform: &Platform, state: &mut State) -> anyhow::Result<()> {
    let entry = util::resolve_entry(e, platform)?;
    Ok(())
}
