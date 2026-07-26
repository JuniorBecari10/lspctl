use crate::{
    operations::util,
    registry::model::{Entry, Platform},
};

pub fn install_pkg(e: Entry, platform: &Platform) -> anyhow::Result<()> {
    let entry = util::resolve_entry(e, platform)?;
    Ok(())
}
