package util

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func AskConfirmation(prompt string) bool {
    fmt.Printf("%s (y/n) ", prompt)

    reader := bufio.NewReader(os.Stdin)
    line, err := reader.ReadString('\n')
    if err != nil {
        return false
    }

    line = strings.TrimSpace(line)
    return strings.ToLower(line) == "y"
}
