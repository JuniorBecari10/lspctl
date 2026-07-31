use std::collections::HashMap;

use anyhow::Context;

use crate::registry::parser::template::{
    context::ResolveContext,
    eval::eval,
    parser::Parser,
    segment::{Segment, split_segments},
};

pub fn parse_template_hashmap(
    map: HashMap<String, String>,
    ctx: &ResolveContext,
) -> anyhow::Result<HashMap<String, String>> {
    map.into_iter()
        .map(|(k, v)| Ok((parse_template(k, ctx)?, parse_template(v, ctx)?)))
        .collect()
}

// source.bin is only used in js-debug-adapter.
// s is an owned string to simplify the parser implementation,
// since the object template string is meant to be moved into the function.
pub fn parse_template(template: String, ctx: &ResolveContext) -> anyhow::Result<String> {
    if !template.contains("{{") {
        // no-op if no templates are present
        return Ok(template);
    }

    let ctx_json = serde_json::to_value(ctx)
        .context("Failed to serialize source context for template resolution")?;

    let mut out = String::with_capacity(template.len());
    for seg in split_segments(&template)? {
        match seg {
            Segment::Text(t) => out.push_str(t),
            Segment::Expr(inner) => {
                let expr = Parser::new(inner)?.parse_pipeline()?;
                out.push_str(&eval(&expr, &ctx_json)?.into_str()?);
            }
        }
    }

    Ok(out)
}
