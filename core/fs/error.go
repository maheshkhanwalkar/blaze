package fs

import (
	"fmt"
	"os"
)

func check(err error) {
	if err != nil {
		fmt.Printf("Error while processing fs object: %s", err)
		os.Exit(1)
	}
}

func close(f *os.File) {
	err := f.Close()
	check(err)
}
