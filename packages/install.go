package packages

import (
	"lspctl/parse"
	"lspctl/print"
)

func InstallPackages(pkgs []parse.InstallYaml) error {
    for _, pkg := range pkgs {
        if err := install(pkg); err != nil {
            return err
        }
    }

    return nil
}

func install(pkg parse.InstallYaml) error {
    var installFn func(parse.InstallYaml) error
    switch pkg.Source.PkgSource {
        case parse.SourceNpm:
            installFn = installNpm
        case parse.SourceGo:
            installFn = installGo
        case parse.SourceGitHub:
            installFn = installGitHub
        case parse.SourceGeneric:
            installFn = installGeneric

        default:
            print.Fail("Unknown package source.")
    }

    return installFn(pkg)
}

// ---

func installNpm(pkg parse.InstallYaml) error {
    
}

func installGo(pkg parse.InstallYaml) error {
    
}
func installGitHub(pkg parse.InstallYaml) error {
    
}

func installGeneric(pkg parse.InstallYaml) error {
    
}
