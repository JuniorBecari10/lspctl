use std::{fmt::Display, path};

use colored::{ColoredString, Colorize};
use indicatif::ProgressStyle;

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

/// `--> message`: end marker for a step that didn't end well.
#[macro_export]
macro_rules! end_error {
    ($($arg:tt)*) => {
        eprintln!(
            " {} {}",
            colored::Colorize::bold(colored::Colorize::red("-->")),
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

/// ` * message`: header, usually at the start of a list. starts with '\n'
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

// ---

pub trait Fatal<T> {
    fn fatal(self, message: &str) -> T;
}

impl<T, E: Display> Fatal<T> for Result<T, E> {
    fn fatal(self, message: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => fatal!("{message}: {e}."),
        }
    }
}

impl<T> Fatal<T> for Option<T> {
    fn fatal(self, message: &str) -> T {
        match self {
            Some(t) => t,
            None => fatal!("{message}."),
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

// ---

pub trait Format {
    fn verb(&self) -> ColoredString;
    fn quote(&self) -> ColoredString;
    fn url(&self) -> ColoredString;
}

impl Format for str {
    fn verb(&self) -> ColoredString {
        self.cyan().bold()
    }

    fn quote(&self) -> ColoredString {
        format!("'{}'", self.italic()).dimmed()
    }

    fn url(&self) -> ColoredString {
        self.blue().underline()
    }
}

impl<'a> Format for path::Display<'a> {
    fn verb(&self) -> ColoredString {
        self.to_string().verb()
    }

    fn quote(&self) -> ColoredString {
        self.to_string().quote()
    }

    fn url(&self) -> ColoredString {
        self.to_string().url()
    }
}

// ---

pub fn get_progress_bar(verb: &str) -> anyhow::Result<ProgressStyle> {
    Ok(ProgressStyle::with_template(&format!(
        "     {} [{{bar:40.cyan/blue}}] {{percent:.cyan}}{} ({{bytes}}/{{total_bytes}}) {{eta:.green}}",
        verb.verb(),
        "%".cyan()
    ))?
    .progress_chars("━━—"))
}
