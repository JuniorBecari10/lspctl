#[macro_export]
macro_rules! def_consts {
    ($($name: ident = $value: expr),+ $(,)?) => {
        $(
            pub const $name: &str = $value;
        )+
    };
}

const BANNER_ART: &str = r#"
  _                _   _ 
 | |___ _ __   ___| |_| |
 | / __| '_ \ / __| __| |
 | \__ \ |_) | (__| |_| |
 |_|___/ .__/ \___|\__|_|
       |_|               "#;

def_consts!(
    APP_NAME = "lspctl",
    APP_VERSION = "beta 1.0",
    APP_DESC = "A standalone, cross-platform package manager for LSP servers, DAP servers, linters, and formatters.",
    BANNER = BANNER_ART,
);
