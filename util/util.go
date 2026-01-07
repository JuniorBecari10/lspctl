package util

import (
	"fmt"
	"os"
)

func Fail(message string) {
    fmt.Fprintf(os.Stderr, "Error: %s", message)
}

