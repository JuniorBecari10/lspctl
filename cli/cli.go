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
    args := os.Args[2:]
    
    switch subcommand {
        case "help": usage()
        case "install", "i": installCmd(args)
        case "remove", "rm", "r": // TODO: remove (flag yes)
        case "list", "ls", "l": // TODO: list (flag filters)
        case "search", "s": // TODO: search (flag filters)
        case "update", "up", "u": // TODO: update
        case "delete-reg", "dr": // TODO: delete registry (flag yes)
        case "destroy": // TODO: destroy all data (flag yes) (add only "delete all packages but not the registry"?)
    }
}
