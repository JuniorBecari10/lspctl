package cli

import (
	"os"
)

func Cli() {
    if len(os.Args) < 2 {
        // enter interactive mode or display usage info
        errorUsage()
        os.Exit(1)
    }

    subcommand := os.Args[1]
    switch subcommand {
        case "help": explicitUsage()
        case "install", "i": // TODO: install (flag --update or -u)
        case "remove", "rm", "r": // TODO: remove
        case "list", "ls", "l": // TODO: list
        case "search", "s": // TODO: search
        case "update", "up", "u": // TODO: update
    }
}
