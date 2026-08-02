use crate::registry::{
    model::{Entry, Platform, ResolvedEntry},
    parser::template::{self, context},
};

pub fn resolve_entry(e: Entry, host: &Platform) -> anyhow::Result<ResolvedEntry> {
    let ctx = context::build_context(&e.source, host)?;
    let entry = template::resolve_entry(e, &ctx)?;

    Ok(entry)
}
