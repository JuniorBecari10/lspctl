package util

import (
	"fmt"
	"os"
)

func Fail(format string, args ...any) {
    fmt.Fprintf(os.Stderr, "Error: %s\n", fmt.Sprintf(format, args...))
    os.Exit(1)
}

func Log(format string, args ...any) {
	// for now it's just a println with formatting. but we can extend it
	fmt.Printf("%s\n", fmt.Sprintf(format, args...))
}
