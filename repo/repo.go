package repo

import (
	"lspctl/util"
	"os"
	"os/exec"
	"path/filepath"
)

const (
    AppDirName = "lspctl"
    RegistryDirName = "registry"
    PackagesDirName = "packages"
    BinDirName = "bin"
)

const MasonRegistryRepo = "https://github.com/mason-org/mason-registry"

func AppDir() string {
    // get the folder from the variable
    if dir := os.Getenv("XDG_DATA_HOME"); dir != "" {
        return filepath.Join(dir, AppDirName)
    }

    // the variable doesn't exist. get from absolute path
    home, err := os.UserHomeDir()
    if err != nil {
        util.Fail("Cannot determine home directory")
    }

    return filepath.Join(home, ".local", "share", AppDirName)
}

func RegistryDir() string {
    return filepath.Join(AppDir(), RegistryDirName)
}

func PackagesDir() string {
    return filepath.Join(AppDir(), PackagesDirName)
}

func BinDir() string {
    return filepath.Join(AppDir(), BinDirName)
}

// ---

func CheckRepo() {
    
}

func CloneRepo() {
    cmd := exec.Command("git", "clone", MasonRegistryRepo, RegistryDir())

}
