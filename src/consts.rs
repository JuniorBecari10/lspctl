macro_rules! new_const {
    ($name: ident = $value: expr) => {
        pub const $name: &str = $value;
    };
}

new_const!(APP_NAME = "lspctl");
new_const!(APP_VERSION = "alpha v0.1");
new_const!(APP_DESC = "A TUI and CLI tool to manage installed LSPs, based on Mason's repository.");

new_const!(BIN_DIR = "bin");
new_const!(REGISTRY_DIR = "registry");
new_const!(PACKAGES_DIR = "packages");
