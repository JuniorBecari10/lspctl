use colored::{ColoredString, Colorize};

pub fn display_width(s: &str) -> usize {
    s.chars().count()
}

pub fn rule(len: usize) -> ColoredString {
    "─".repeat(len).dimmed()
}
