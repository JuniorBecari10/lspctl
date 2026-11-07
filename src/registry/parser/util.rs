use anyhow::Context;
use percent_encoding::percent_decode_str;

use crate::log::Format;

// this is to fix a bug in packageurl's version parsing.
// so we hand-roll our own implementation of the version parsing algorithm!
pub fn extract_version(raw_id: &str) -> anyhow::Result<String> {
    let at_idx = raw_id
        .rfind('@')
        .ok_or_else(|| anyhow::anyhow!("purl {} has no version segment", raw_id.quote()))?;

    let after = &raw_id[at_idx + 1..];
    let version_raw = after.split(['?', '#']).next().unwrap_or(after);

    percent_decode_str(version_raw)
        .decode_utf8()
        .map(|s| s.into_owned())
        .with_context(|| {
            format!(
                "failed to percent-decode version in purl {}",
                raw_id.quote()
            )
        })
}
