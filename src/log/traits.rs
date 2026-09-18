use std::{fmt::Display, path};

use colored::{ColoredString, Colorize};

use crate::{error, fatal};

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
