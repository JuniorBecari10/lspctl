use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Deprecation {
    since: String,
    message: String,
}
