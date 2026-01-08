package cli

import (
	"fmt"
	"io"
	"os"
)

func errorUsage() {
    fmt.Fprintln(os.Stderr, "No command provided.")
    rawUsage(os.Stderr)
}

func usage() {
    rawUsage(os.Stdout)
}

func rawUsage(w io.Writer) {
    fmt.Fprintf(w, "lspctl usage\n\n")
}
