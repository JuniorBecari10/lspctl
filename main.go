package main

import (
	"lspctl/repo"
	"os"
)

func main() {
    repo.CheckRepo(len(os.Args) > 1)
}
