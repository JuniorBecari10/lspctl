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
