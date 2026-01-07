package install

import "lspctl/parse"

func Install(yml parse.InstallYaml) {
    switch yml.Source.PkgSource {
        case parse.SourceNpm:
            installNpm(yml)
    }
}

// ---

func installNpm(yml parse.InstallYaml) {
    
}
