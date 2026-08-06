use std::fmt::Display;

/// `--> message`: major step marker. writes a '\n' before it.
#[macro_export]
macro_rules! step {
    ($($arg:tt)*) => {
        eprintln!(
            "\n {} {}",
            colored::Colorize::bold(colored::Colorize::green("-->")),
            format!($($arg)*)
        )
    };
}

/// `    message`: plain sub-detail under a step, no marker, indented.
#[macro_export]
macro_rules! note {
    ($($arg:tt)*) => {
        eprintln!("     {}", format!($($arg)*))
    };
}

/// `--> message`: end marker for a step.
#[macro_export]
macro_rules! end {
    ($($arg:tt)*) => {
        eprintln!(
            " {} {}",
            colored::Colorize::bold(colored::Colorize::blue("-->")),
            format!($($arg)*)
        )
    };
}

/// `error: message`: always to stderr, red + bold prefix.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!(
            "{} {}",
            colored::Colorize::bold(colored::Colorize::red("error:")),
            format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        error!($($arg)*);
        std::process::exit(1)
    }};
}

/// ` * message`: header, usually at the start of a list. writes a '\n' before it.
#[macro_export]
macro_rules! header {
    ($($arg:tt)*) => {
        eprintln!(
            "\n {} {}",
            colored::Colorize::bold(colored::Colorize::blue("*")),
            format!($($arg)*)
        )
    };
}

/// `  - message`: one line in a list (e.g. `lspctl list`), dim bullet.
#[macro_export]
macro_rules! list {
    ($($arg:tt)*) => {
        println!(
            "   {} {}",
            colored::Colorize::dimmed("-"),
            format!($($arg)*)
        )
    };
}

pub trait Fatal<T> {
    fn fatal(self, message: &str) -> T;
}

impl<T> Fatal<T> for anyhow::Result<T> {
    fn fatal(self, message: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => fatal!("{message}: {e}."),
        }
    }
}

pub trait LogPretty<T> {
    fn log(self, f: impl Fn() -> String) -> String
    where
        T: Display;
}

impl<T> LogPretty<T> for Option<T> {
    fn log(self, f: impl Fn() -> String) -> String
    where
        T: Display,
    {
        self.map_or_else(f, |t| t.to_string())
    }
}
