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

// Check the registry local repository.
// If it exists, use the provided flag to perform the pull (i.e. update the local "cache") or not.
// If it doesn't, perform a clone anyway.
func CheckRepo(pull bool) {
    if util.DirExists(RegistryDir()) {
        util.Log("Local registry exists. %s", PullMsg(pull))

        if pull {
            PullRepo()
            util.Log("Pull complete.")
        }
    } else {
        util.Log("Local registry doesn't exist. Cloning...")
        CloneRepo()
        util.Log("Clone complete.")
    }
}

func PullRepo() {
    err := exec.Command("git", "-C", RegistryDir(), "fetch").Run()
    if err != nil {
        util.Fail("Failed to fetch repo: %s", err)
    }

    err = exec.Command("git", "-C", RegistryDir(), "pull", "--ff-only").Run()
    if err != nil {
        util.Fail("Failed to pull repo: %s", err)
    }
}

func CloneRepo() {
    err := exec.Command("git", "clone", MasonRegistryRepo, RegistryDir()).Run()
    if err != nil {
        util.Fail("Failed to clone repo: %s", err)
    }
}

// ---

func PullMsg(pull bool) string {
    if pull {
        return "Pulling changes..."
    } else {
        return "Not pulling changes."
    }
}
