use crate::registry::{
    model::{Entry, Platform},
    parser::template::{context, resolve_entry},
};

pub fn install_pkg(entry: &Entry, platform: &Platform) -> anyhow::Result<()> {
    let ctx = context::build_context(&entry.source, platform)?;
    let resolved = resolve_entry()
    Ok(())
}
