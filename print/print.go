package print

import (
	"fmt"
	"os"
)

func Fail(message string) {
	Failf("%s", message)
}

func Failf(format string, args ...any) {
    fmt.Fprintf(os.Stderr, "Error: %s\n", fmt.Sprintf(format, args...))
    os.Exit(1)
}

func Log(message string) {
	Logf("%s", message)
}

func Logf(format string, args ...any) {
	// for now it's just a println with formatting. but we can extend it
	fmt.Printf("%s\n", fmt.Sprintf(format, args...))
}
