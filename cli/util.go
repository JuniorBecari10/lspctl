package cli

import (
	"flag"
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

func flagYes(fs *flag.FlagSet) *bool {
    yes := fs.Bool("yes", false, "Assume yes for all prompts")
    fs.BoolVar(yes, "y", false, "Alias for --yes")

    return yes
}
