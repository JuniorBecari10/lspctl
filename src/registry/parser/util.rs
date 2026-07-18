use crate::registry::model::Platform;

pub fn get_platform(s: &str) -> Option<Platform> {
    use Platform::*;

    match s {
        "unix" => Some(Unix),
        "darwin" => Some(Darwin),
        "linux" => Some(Linux),
        "win" => Some(Windows),
        _ => None,
    }
}
