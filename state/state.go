package state

import (
	"lspctl/parse"
	"time"
)

type State struct {
    Installed map[string]Package
}

type Package struct {
    Name string
    Source parse.PackageSource
    Version string

    Bins []string
    InstallDir string

    InstalledAt time.Time
    Dependencies []string
}
