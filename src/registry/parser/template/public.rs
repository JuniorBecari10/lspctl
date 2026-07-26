use std::collections::HashMap;

use anyhow::Context;

use crate::registry::{
    model::Source,
    parser::template::{
        eval::eval,
        parser::Parser,
        segment::{Segment, split_segments},
    },
};

pub fn parse_template_hashmap(
    map: HashMap<String, String>,
    source: &Source,
) -> anyhow::Result<HashMap<String, String>> {
    map.into_iter()
        .map(|(k, v)| Ok((parse_template(k, source)?, parse_template(v, source)?)))
        .collect()
}

// source.bin is only used in js-debug-adapter.
// s is an owned string to simplify the parser implementation,
// since the object template string is meant to be moved into the function.
pub fn parse_template(template: String, source: &Source) -> anyhow::Result<String> {
    if !template.contains("{{") {
        // no-op if no templates are present
        return Ok(template);
    }

    let source_json = serde_json::to_value(source)
        .context("Failed to serialize source for template resolution")?;

    let mut out = String::with_capacity(template.len());
    for seg in split_segments(&template)? {
        match seg {
            Segment::Text(t) => out.push_str(t),
            Segment::Expr(inner) => {
                let expr = Parser::new(inner)?.parse_pipeline()?;
                out.push_str(&eval(&expr, &source_json)?.into_str()?);
            }
        }
    }

    dbg!(&template, &source_json, &out);
    Ok(out)
}
