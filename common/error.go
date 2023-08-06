package common

import (
	"fmt"
	"os"
)

func Check(err error) {
	if err != nil {
		fmt.Printf("Error while processing fs object: %s", err)
		os.Exit(1)
	}
}

func CloseFile(f *os.File) {
	err := f.Close()
	Check(err)
}
