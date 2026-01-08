package cli

import (
	"flag"
	"lspctl/print"
	"io"
)

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
    
    yes := flagYes(fs)

    msg := "Turn both flags on"
    both := fs.Bool("uy", false, msg)
    fs.BoolVar(both, "yu", false, msg)

    return sync, yes, both
}

// ---


