package cli

import (
	"lspctl/print"
	"lspctl/repo"
	"lspctl/util"
)

func install(packages []string, sync, yes bool) {
    repo.CheckRepo(sync)

    // TODO: check if the package exists, fetching its yaml and if it doesn't exist, the package doesn't exist
    
    print.Log("\nPackages to be installed:")

    for _, pkg := range packages {
        print.Logf("- %s", pkg)
    }

    print.Log("")

    if !yes && !util.AskConfirmation("Do you want to continue?") {
        return
    }

    print.Log("Installing!")
    // TODO: install
}
