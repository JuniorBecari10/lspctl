package cli

import (
	"lspctl/packages"
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
    
    print.Logf("\nPackages to be installed (%d):", len(pkgs))

    for _, pkg := range pkgs {
        print.Logf("- %s (%s)", pkg.Name, pkg.Source.PkgSource)

        for _, extra := range pkg.Source.ExtraPkgs {
            print.Logf("    - %s", extra)
        }
    }

    print.Log("")

    if !yes && !util.AskConfirmation("Do you want to continue?") {
        return
    }

    if err = packages.InstallPackages(pkgs); err != nil {
        print.Fail(err.Error())
    }
}
