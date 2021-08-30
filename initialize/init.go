package initialize

import (
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
	// FIXME: need to add robust checks -- what if subdirectories are missing or
	//  there's some sort of corruption?
	if exists(".blaze") {
		fmt.Println("Repository already initialized, skipping...")
		return
	}

	createDir(".blaze")

	// Create all required directories under .blaze/
	sub := []string{"staging", "scratch", "index", "slice", "object"}
	for _, dir := range sub {
		createDir(fmt.Sprintf(".blaze/%s", dir))
	}
}
