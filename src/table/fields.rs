use std::fmt::Display;

use colored::Colorize;

use super::util;

// Aligned 'Label:  value' lines. The label column is sized to the longest
// label automatically, so no hand-tuned width constants.
pub struct Fields {
    entries: Vec<(String, String)>,
    indent: usize,
    gap: usize,
}

impl Default for Fields {
    fn default() -> Self {
        Self::new()
    }
}

impl Fields {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            indent: 2,
            gap: 2,
        }
    }

    pub fn field(&mut self, label: impl Into<String>, value: impl Display) -> &mut Self {
        self.entries.push((label.into(), value.to_string()));
        self
    }

    // Comma-separated list, or a highlighted placeholder when empty.
    pub fn list<I>(&mut self, label: impl Into<String>, items: I, empty: &str) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Display,
    {
        let items: Vec<String> = items.into_iter().map(|i| i.to_string()).collect();
        if items.is_empty() {
            self.field(label, empty.red().bold())
        } else {
            self.field(label, items.join(", "))
        }
    }

    pub fn print(&self) {
        print!("{}", self.render());
    }

    pub fn render(&self) -> String {
        // +1 for the trailing colon
        let label_width = self
            .entries
            .iter()
            .map(|(label, _)| util::display_width(label))
            .max()
            .unwrap_or(0)
            + 1;

        let indent = " ".repeat(self.indent);
        let gap = " ".repeat(self.gap);

        let mut out = String::new();

        for (label, value) in &self.entries {
            let label = format!("{:<label_width$}", format!("{label}:"));
            out.push_str(&format!("{indent}{}{gap}{value}\n", label.dimmed()));
        }
        out
    }
}
