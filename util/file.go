package util

import (
	"os"
	"path/filepath"
)

func DirExists(path string) bool {
    info, err := os.Stat(path)
    return err == nil && info.IsDir()
}

func IsGitRepo(dir string) bool {
    return DirExists(filepath.Join(dir, ".git"))
}
