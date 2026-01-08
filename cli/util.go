package cli

import (
	"fmt"
	"io"
	"os"
)

func errorUsage() {
    usage(os.Stderr)
}

func explicitUsage() {
    usage(os.Stdout)
}

func usage(w io.Writer) {
    fmt.Fprintf(w, "lspctl usage\n\n")
}
