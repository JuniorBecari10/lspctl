use std::collections::HashMap;

use crate::registry::model::Source;

pub fn parse_template_hashmap(
    map: HashMap<String, String>,
    source: &Source,
) -> anyhow::Result<HashMap<String, String>> {
    map.into_iter()
        .map(|(k, v)| Ok((parse_template(k, source)?, parse_template(v, source)?)))
        .collect()
}

// source.bin is only used in js-debug-adapter
// s is an owned string to simplify the parser implementation,
// since the object template string is meant to be moved into the function
// version is 'source.purl.version'
pub fn parse_template(template: String, source: &Source) -> anyhow::Result<String> {
    if !template.contains("{{") {
        return Ok(template);
    }
}
