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
        case parse.SourcePypi:
            installFn = installPypi
        default:
            print.Fail("Unknown package source.")
    }

    return installFn(pkg)
}

// ---

func installNpm(pkg parse.InstallYaml) error {
    print.Fail("npm not supported yet")
    return nil
}

func installGo(pkg parse.InstallYaml) error {
    print.Fail("go not supported yet")
    return nil
}
func installGitHub(pkg parse.InstallYaml) error {
    print.Fail("github not supported yet")
    return nil
}

func installGeneric(pkg parse.InstallYaml) error {
    print.Fail("generic not supported yet")
    return nil
}

func installPypi(pkg parse.InstallYaml) error {
    print.Fail("pypi not supported yet")
    return nil
}
