package core

import (
	"crypto/sha256"
	"fmt"
	"os"
)

// TrackedObject represents (as the name implies) a tracked object within the
// version control system. The object is a snapshot in time of some file within the repository
type TrackedObject struct {
	name string
	data []byte
}

// GenerateName creates an object name using the provided path name
// as input to the underlying generation logic
func GenerateName(path string) string {
	hash := sha256.Sum256([]byte(path))
	return fmt.Sprintf("%x", hash)
}

func check(err error) {
	if err != nil {
		fmt.Printf("Error while processing object: %s", err)
		os.Exit(1)
	}
}

// ToDisk serializes the provided object to disk
func ToDisk(obj *TrackedObject) {
	path := fmt.Sprintf(".blaze/object/%s", obj.name)
	f, err := os.Create(path)
	check(err)

	// Only need to write the data to disk, since the object name is encoded
	// within the file name
	_, err = f.Write(obj.data)
	check(err)
	err = f.Close()
	check(err)
}

// FromDisk deserializes the object from disk
func FromDisk(name string) *TrackedObject {
	path := fmt.Sprintf(".blaze/object/%s", name)
	f, err:= os.Open(path)
	check(err)

	info, err := f.Stat()
	check(err)

	buffer := make([]byte, info.Size())
	_, err = f.Read(buffer)
	check(err)

	obj := TrackedObject{name: name, data: buffer}
	return &obj
}
