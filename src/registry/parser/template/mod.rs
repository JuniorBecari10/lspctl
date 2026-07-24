pub mod parser;
mod token;

pub use parser::*;

use crate::registry::model::{Entry, Registry, Source};

pub fn resolve_templates(reg: Registry) -> anyhow::Result<Registry> {
    Ok(Registry(
        reg.0
            .into_iter()
            .map(resolve_entry)
            .collect::<anyhow::Result<_>>()?,
    ))
}

fn resolve_entry(e: Entry) -> anyhow::Result<Entry> {
    let source = resolve_source(e.source)?;

    Ok(Entry {
        bin: e.bin.map(|b| parser::parse_template_hashmap(b, &source)),
        source,
        ..e
    })
}

fn resolve_source(s: Source) -> anyhow::Result<Source> {
    Ok(Source {
        bin: s.bin.map(|b| parser::parse_template(b, &s)),
        ..s
    })
}
