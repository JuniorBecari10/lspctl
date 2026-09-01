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
    APP_VERSION = "beta 1.0",
    APP_DESC = "A standalone, cross-platform package manager for LSP servers, DAP servers, linters, and formatters.",
);
