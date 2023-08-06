package main

import (
	"blaze/core/fs"
	"fmt"
	"os"
)

func createDir(dir string) {
	err := os.Mkdir(dir, 0755)

	if err != nil {
		fmt.Printf("Initialization failure: could not create dir %s: %s\n", dir, err)
		os.Exit(1)
	}
}

func exists(dir string) bool {
	if _, err := os.Stat(dir); err != nil {
		if os.IsNotExist(err) {
			return false
		} else {
			fmt.Printf("Initialization failure: stat: %s\n", err)
		}
	}

	return true
}

func Init() {
	if exists(".blaze") {
		fmt.Println("Repository already initialized, skipping...")
		return
	}

	createDir(".blaze")

	// Create all required directories under .blaze/
	sub := []string{"blob", "inode"}
	for _, dir := range sub {
		createDir(fmt.Sprintf(".blaze/%s", dir))
	}

	// Create superblock
	sb := fs.NewSuperBlock()
	sb.ToDisk()
}

func main() {
	Init()
}
