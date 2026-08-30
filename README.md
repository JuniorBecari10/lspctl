# lspctl

A standalone, cross-platform package manager for LSP servers, DAP adapters,
linters, and formatters. Installs from the [Mason](https://github.com/mason-org/mason-registry)
registry without depending on Neovim or any specific editor.

Version: **Beta 1.0**.

## Motivation

A long time ago I primarily used Neovim for coding, and I used Mason for installing the tools I needed.
Now, I use [Helix](https://helix-editor.com/), and now I am unable to use Mason again for installing those tools.
From this friction, `lspctl` was born, with the aim to provide a single and standardized way for installing
LSP servers, DAP adapters, linters and formatters for _any editor_, by exposing a single binary for each
installed tool to be used by your editor.

## Why this name?

`lspctl` was chosen to mirror the name of [systemd](https://systemd.io/)'s command `systemctl`,
which controls the running services, but now for LSPs and more.

## Status

`lspctl` is under active development, with core operations such as _install_, _remove_, _list_ and _search_
working reliably, by having been tested across several packages from the registry. Most of the packages are
already supported; see the table below for the current state of the implementation:

### Current state

#### Interfaces

- [x] CLI
- [ ] TUI
- [ ] GUI

#### CLI

- [x] Install
- [x] Remove
- [x] List
- [x] Search
- [x] Info
- [x] Delete
- [ ] Update

#### Install Targets

- [x] npm
- [x] Go
- [x] Cargo
- [x] NuGet
- [x] Gem
- [x] PyPI
- [x] LuaRocks
- [ ] Composer
- [ ] Opam
- [x] GitHub Assets
- [ ] Download
- [ ] Build

## Usage

```sh
# install one or more packages
lspctl install gopls rust-analyzer
lspctl i gopls rust-analyzer   # short alias

# remove one or more packages
lspctl remove gopls
lspctl remove --all            # remove everything installed

# list packages
lspctl list                    # everything in the registry
lspctl list --installed        # only what's installed
lspctl list --verbose          # multi-line detail per package

# search by name (regex)
lspctl search '^rust'
lspctl search debug --installed

# skip confirmation prompts
lspctl install gopls --yes
```

For more information, run `lspctl --help`.

## How it works

The source of truth for `lspctl` is the Mason registry.
Everything it does is described there. <br />

A quick summary of the steps:

1. The registry is fetched from the GitHub Releases, and cached locally as a JSON file;
2. The package definitions use a small DSL for templating, so `lspctl` has an embedded parser for them.
   It lexes, parses and evaluates the expressions to generate a ready-to-use registry to be used to install your packages;
3. `lspctl` scans all the entries of the packages you want to install and selects the most specific one that matches your system;
4. All steps are done atomically. Installation is done in a temporary folder first, and then moved atomically to its final location.
   When removing packages it also performs the action atomically. This is done to prevent half states.
5. Some packages need _shims_, which are small wrapper scripts that run the installed tool for you, in case it doesn't have an executable
   file ready to be used;
6. `lspctl` keeps a file lock to prevent two or more instances from running at the same time.

## Building from source

Requires a recent Rust toolchain (1.89+).

```sh
git clone https://github.com/JuniorBecari10/lspctl
cd lspctl
cargo build --release
```

## Supported platforms

Linux, macOS and Windows. It does platform-specific behavior for each one of them, so that the installed tool runs flawlessly.
All of them should be supported, though all the testing so far has been done on Linux.

## Contributing

Contributions are welcome. You can do so by doing Pull Requests and opening Issues.
If you notice any undesirable behavior, please don't hesitate and open an Issue about it.
By doing so, you will help making `lspctl` better for everyone.

## License

See `LICENSE`.
