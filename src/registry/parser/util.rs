use anyhow::Context;
use percent_encoding::percent_decode_str;

pub fn extract_version(raw_id: &str) -> anyhow::Result<String> {
    let at_idx = raw_id
        .rfind('@')
        .ok_or_else(|| anyhow::anyhow!("purl '{raw_id}' has no version segment"))?;

    let after = &raw_id[at_idx + 1..];
    let version_raw = after.split(['?', '#']).next().unwrap_or(after);

    percent_decode_str(version_raw)
        .decode_utf8()
        .map(|s| s.into_owned())
        .with_context(|| format!("failed to percent-decode version in purl '{raw_id}'"))
}
