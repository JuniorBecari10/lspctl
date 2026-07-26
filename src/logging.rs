use colog::format::CologStyle;
use colored::Colorize;
use log::Level;

pub struct LogStyle;

impl CologStyle for LogStyle {
    fn level_token(&self, level: &Level) -> &str {
        match level {
            Level::Error => "[error]",
            Level::Warn => "[warn] ",
            Level::Info => "    -->",
            Level::Debug => "[debug]",
            Level::Trace => "[trace]",
        }
    }

    fn level_color(&self, level: &Level, msg: &str) -> String {
        match level {
            Level::Error => msg.red().bold(),
            Level::Warn => msg.yellow().bold(),
            Level::Info => msg.green().bold(),
            Level::Debug => msg.blue(),
            Level::Trace => msg.purple(),
        }
        .to_string()
    }

    fn prefix_token(&self, level: &Level) -> String {
        self.level_color(level, self.level_token(level))
    }
}

pub fn init() {
    colog::basic_builder()
        .format(colog::formatter(LogStyle))
        .filter_level(log::LevelFilter::Info)
        .init();
}
