package main

import (
    "fmt"
    "lspctl/parse"
)

// ---

var file = `---
name: svelte-language-server
description: A language server (implementing the language server protocol) for Svelte.
homepage: https://github.com/sveltejs/language-tools
licenses:
  - MIT
languages:
  - Svelte
categories:
  - LSP

source:
  id: pkg:npm/svelte-language-server@0.17.23
  extra_packages:
    - typescript-svelte-plugin

bin:
  svelteserver: npm:svelteserver

neovim:
  lspconfig: svelte`

func main() {
    parsed, err := parse.ParseYaml(file)
    if err != nil {
        panic(err)
    }

    install, err := parse.ConvertInstall(parsed)
    if err != nil {
        panic(err)
    }
    
	fmt.Printf("%#v\n\n", parsed)
	fmt.Printf("%#v\n", install)
}
