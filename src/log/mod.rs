mod macros;
mod traits;

use colored::Colorize;
use indicatif::ProgressStyle;

pub use traits::*;

pub fn get_progress_bar(verb: &str) -> anyhow::Result<ProgressStyle> {
    Ok(ProgressStyle::with_template(&format!(
        "     {} [{{bar:40.cyan/blue}}] {{percent:.cyan}}{} ({{bytes}}/{{total_bytes}}) {{eta:.green}}",
        verb.verb(),
        "%".cyan()
    ))?
    .progress_chars("━━—"))
}
