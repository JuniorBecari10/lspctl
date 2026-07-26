use crate::registry::parser::template::ast::{Expr, Filter};

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Bool(bool),
}

impl Value {
    pub fn into_str(self) -> anyhow::Result<String> {
        match self {
            Value::Str(s) => Ok(s),
            Value::Bool(b) => anyhow::bail!("Expected string, got bool '{b}'"),
        }
    }

    fn as_bool(&self) -> anyhow::Result<bool> {
        match self {
            Value::Str(s) => anyhow::bail!("Expected bool, got string {s:?}"),
            Value::Bool(b) => Ok(*b),
        }
    }
}
pub fn eval(expr: &Expr, source_json: &serde_json::Value) -> anyhow::Result<Value> {
    match expr {
        Expr::Str(s) => Ok(Value::Str(s.clone())),
        Expr::Path(segs) => resolve_path(segs, source_json).map(Value::Str),
        Expr::Call { name, args } => call_builtin(name, args, source_json),

        Expr::Pipeline { base, filters } => {
            let mut v = eval(base, source_json)?;
            for f in filters {
                v = apply_filter(f, v, source_json)?;
            }
            Ok(v)
        }
    }
}

fn resolve_path(segs: &[String], source_json: &serde_json::Value) -> anyhow::Result<String> {
    if segs.len() == 1 && segs[0] == "version" {
        return resolve_path(&["purl".into(), "version".into()], source_json);
    }

    let rest: &[String] = if segs.first().map(String::as_str) == Some("source") {
        &segs[1..]
    } else {
        segs
    };

    let mut cur = source_json;
    for seg in rest {
        cur = cur
            .get(seg)
            .ok_or_else(|| anyhow::anyhow!("path `{}` not found", segs.join(".")))?;
    }

    match cur {
        serde_json::Value::String(s) => Ok(s.clone()),
        other => anyhow::bail!("path `{}` is not a string: {other}", segs.join(".")),
    }
}

fn call_builtin(
    name: &str,
    args: &[Expr],
    source_json: &serde_json::Value,
) -> anyhow::Result<Value> {
    match name {
        "is_platform" => {
            let want = eval(
                args.first()
                    .ok_or_else(|| anyhow::anyhow!("'is_platform()' needs 1 argument"))?,
                source_json,
            )?
            .into_str()?;
            Ok(Value::Bool(current_platform_matches(&want)))
        }

        "take_if_not" => {
            let cond = eval(
                args.first()
                    .ok_or_else(|| anyhow::anyhow!("'take_if_not()' needs a condition argument"))?,
                source_json,
            )?
            .as_bool()?;
            let value = args
                .get(1)
                .ok_or_else(|| anyhow::anyhow!("'take_if_not()' needs a value argument"))?;
            if cond {
                Ok(Value::Str(String::new()))
            } else {
                eval(value, source_json)
            }
        }
        other => anyhow::bail!("unknown function `{other}`"),
    }
}

fn apply_filter(
    f: &Filter,
    input: Value,
    source_json: &serde_json::Value,
) -> anyhow::Result<Value> {
    match f.name.as_str() {
        "strip_prefix" => {
            let prefix = eval(
                f.args
                    .first()
                    .ok_or_else(|| anyhow::anyhow!("'strip_prefix' requires 1 argument"))?,
                source_json,
            )?
            .into_str()?;

            let s = input.into_str()?;
            Ok(Value::Str(
                s.strip_prefix(prefix.as_str()).unwrap_or(&s).to_string(),
            ))
        }

        // filter form: `'yq.1' | take_if_not(is_platform('win'))`: piped
        // value *is* the "value" arg, only condition is passed explicitly
        "take_if_not" => {
            let cond = eval(
                f.args.first().ok_or_else(|| {
                    anyhow::anyhow!("'take_if_not()' requires a condition argument")
                })?,
                source_json,
            )?
            .as_bool()?;

            if cond {
                Ok(Value::Str(String::new()))
            } else {
                Ok(input)
            }
        }

        other => anyhow::bail!("Unknown filter `{other}`"),
    }
}

fn current_platform_matches(name: &str) -> bool {
    let os = std::env::consts::OS;

    match name {
        "win" | "windows" => os == "windows",
        "mac" | "macos" => os == "macos",
        "linux" => os == "linux",
        other => other == os,
    }
}
