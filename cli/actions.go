package cli

import (
	"lspctl/print"
	"lspctl/repo"
	"lspctl/util"
)

func install(names []string, sync, yes bool) {
    repo.CheckRepo(sync)

    pkgs, err := repo.GetPackages(names)
    if err != nil {
        print.Fail(err.Error())
    }
    
    print.Log("\nPackages to be installed:")

    for _, pkg := range pkgs {
        print.Logf("- %s", pkg.Name)
    }

    print.Log("")

    if !yes && !util.AskConfirmation("Do you want to continue?") {
        return
    }

    print.Log("Installing!")
    // TODO: install
}
