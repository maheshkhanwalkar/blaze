package file

import (
	"fmt"
	"google.golang.org/protobuf/proto"
	"os"
)

func check(err error) {
	if err != nil {
		fmt.Printf("blaze: file error: %s", err)
		os.Exit(1)
	}
}

// LoadFile reads the entire contents of the file specified by
// path and returns it as a string
func LoadFile(path string) string {
	return string(LoadBinaryFile(path))
}

// LoadBinaryFile reads the entire contents of the file specified by
// path and returns the raw bytes
func LoadBinaryFile(path string) []byte {
	f, err := os.Open(path)
	check(err)

	info, err := f.Stat()
	check(err)

	buffer := make([]byte, info.Size())
	_, err = f.Read(buffer)
	check(err)

	return buffer
}

func Serialise(m proto.Message, path string) {
	output, err := proto.Marshal(m)
	check(err)
	err = os.WriteFile(path, output, 0755)
	check(err)
}
