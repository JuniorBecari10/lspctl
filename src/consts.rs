#[macro_export]
macro_rules! def_consts {
    ($($name: ident = $value: expr),+ $(,)?) => {
        $(
            pub const $name: &str = $value;
        )+
    };
}

def_consts!(
    APP_NAME = "lspctl",
    APP_VERSION = "alpha v0.1",
    APP_DESC = "A TUI and CLI tool to manage installed LSPs, based on Mason's repository.",
);
