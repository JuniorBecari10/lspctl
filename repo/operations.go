package repo

import (
	"lspctl/parse"
)

func getPackage(name string) (parse.InstallYaml, error) {
    yaml, err := GetPackageYaml(name)
    if err != nil {
        return parse.InstallYaml{}, err
    }

    parsed, err := parse.ParseYaml(yaml)
    if err != nil {
        return parse.InstallYaml{}, err
    }

    install, err := parse.ConvertInstall(parsed)
    if err != nil {
        return parse.InstallYaml{}, err
    }

    return install, nil
}

func GetPackages(names []string) ([]parse.InstallYaml, error) {
    res := []parse.InstallYaml{}
    for _, name := range names {
        pkg, err := getPackage(name)
        if err != nil {
            return nil, err
        }

        res = append(res, pkg)
    }

    return res, nil
}

