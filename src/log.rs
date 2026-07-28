/// `--> message`: major step marker. writes a '\n' before it.
#[macro_export]
macro_rules! step {
    ($($arg:tt)*) => {
        println!(
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
        println!("     {}", format!($($arg)*))
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

/// ` * message`: header, usually at the start of a list. writes a '\n' before it.
#[macro_export]
macro_rules! header {
    ($($arg:tt)*) => {
        println!(
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
