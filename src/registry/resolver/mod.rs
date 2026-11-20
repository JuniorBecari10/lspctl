use crate::registry::{
    model::{Entry, Platform, ResolvedEntry},
    parser::template::{self, context},
};

mod merge;

// TODO: maybe there is a better way to structure this
// the steps are:
// it receives an Entry (got from the registry, then parsed to fit the model better for internal use) and the host platform.
// then,
// - it builds a ResolveContext from the Entry, with the best installation variant for your system, plus some metadata;
// - then, it uses this context to pre-resolve the Entry, which is the same struct with pre-resolved data.
//   -> pre-resolved data means all template texts replaced with actual data (done by the internal parser, which is called there),
//      which uses the ResolveContext built earlier.
// - and lastly, it generates a ResolvedEntry (another struct) merging the pre-resolved data with the host platform, and returns it.
pub fn resolve_entry(e: Entry, host: &Platform) -> anyhow::Result<ResolvedEntry> {
    let ctx = context::build_context(&e.source, host)?;
    let entry = template::pre_resolve_entry(e, &ctx)?;
    let resolved = merge::merge_context(entry, host);

    Ok(resolved)
}
