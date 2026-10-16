use const_format::formatcp;

#[macro_export]
macro_rules! def_consts {
    ($($name: ident = $value: expr),+ $(,)?) => {
        $(
            pub const $name: &str = $value;
        )+
    };
}

const BANNER_ART: &str = formatcp!(
    r#"
  _                _   _ 
 | |___ _ __   ___| |_| |
 | / __| '_ \ / __| __| |
 | \__ \ |_) | (__| |_| |
 |_|___/ .__/ \___|\__|_|    {APP_VERSION}
       |_|               "#
);

def_consts!(
    APP_NAME = "lspctl",
    APP_VERSION = "0.2.0 beta",
    APP_DESC = "A standalone, cross-platform package manager for LSP and DAP servers, linters, and formatters.",
    BANNER = BANNER_ART,
);
