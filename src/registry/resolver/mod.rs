use crate::registry::{
    model::{Entry, Platform, ResolvedEntry},
    parser::template::{self, context},
};

pub fn resolve_entry(e: Entry, platform: &Platform) -> anyhow::Result<ResolvedEntry> {
    let ctx = context::build_context(&e.source, platform)?;

    let entry = template::resolve_entry(e, &ctx)?;
    let asset = ctx.asset.map(serde_json::from_value).transpose()?; // shouldn't fail

    Ok(entry)
}
