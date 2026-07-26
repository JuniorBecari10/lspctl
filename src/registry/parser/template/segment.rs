pub enum Segment<'a> {
    Text(&'a str),
    Expr(&'a str), // slice inside the braces
}

pub fn split_segments(input: &str) -> anyhow::Result<Vec<Segment<'_>>> {
    let mut segments = Vec::new();
    let mut rest = input;

    while let Some(open) = rest.find("{{") {
        if open > 0 {
            segments.push(Segment::Text(&rest[..open]));
        }

        let mut after = &rest[open + 2..];

        // kotlin-lsp's extra '{'.
        // emit as literal text
        if after.starts_with('{') {
            segments.push(Segment::Text(&after[..1]));
            after = &after[1..];
        }

        let close = after
            .find("}}")
            .ok_or_else(|| anyhow::anyhow!("Unterminated {{{{ in template: '{input:?}'"))?;

        segments.push(Segment::Expr(after[..close].trim()));
        rest = &after[close + 2..];
    }

    if !rest.is_empty() {
        segments.push(Segment::Text(rest));
    }

    Ok(segments)
}
