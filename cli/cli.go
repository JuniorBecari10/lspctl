package cli

import (
	"flag"
	"io"
	"lspctl/print"
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
        case "delete-repo", "delr", "dr": // TODO: delete repo (flag yes)
    }
}

func installCmd(args []string) {
    fs := flag.NewFlagSet("install", flag.ContinueOnError)
    fs.SetOutput(io.Discard)

    sync, yes, both := handleInstallFlags(fs)
    
    if err := fs.Parse(args); err != nil {
        print.Fail(err.Error())
    }

    pkgs := fs.Args()
    if fs.NArg() == 0 {
        print.Failf("No packages specified.")
    }

    var realSync, realYes bool
    if *both {
        realSync = true
        realYes = true
    } else {
        realSync = *sync
        realYes = *yes
    }

    install(pkgs, realSync, realYes)
}

func handleInstallFlags(fs *flag.FlagSet) (*bool, *bool, *bool) {
    sync := fs.Bool("sync", false, "Sync registry before installing")
    fs.BoolVar(sync, "s", false, "Alias for --sync")
    
    yes := fs.Bool("yes", false, "Assume yes for all prompts")
    fs.BoolVar(yes, "y", false, "Alias for --yes")

    msg := "Turn both flags on"
    both := fs.Bool("uy", false, msg)
    fs.BoolVar(both, "yu", false, msg)

    return sync, yes, both
}
