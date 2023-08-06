package core

import (
	"blaze/file"
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

// computeHash computes a cryptographic hash of the given data
func computeHash(data []byte) string {
	hash := sha256.Sum256(data)
	return fmt.Sprintf("%x", hash)
}

// generateName creates an object name from the file data
func generateName(data []byte) string {
	return computeHash(data)
}

func check(err error) {
	if err != nil {
		fmt.Printf("Error while processing object: %s", err)
		os.Exit(1)
	}
}

// CreateObject creates a new tracked object from the given data
func CreateObject(data []byte) *TrackedObject {
	name := generateName(data)
	return &TrackedObject{name: name, data: data}
}

// LoadObject loads the tracked object from disk
func LoadObject(name string) *TrackedObject {
	path := fmt.Sprintf(".blaze/object/%s", name)
	buffer := file.LoadBinaryFile(path)

	hash := computeHash(buffer)

	if hash != name {
		fmt.Printf("object tampering detected: %s has been altered", name)
		os.Exit(1)
	}

	return &TrackedObject{name: name, data: buffer}
}

// ToDisk serializes the object to disk
func (obj *TrackedObject) ToDisk() {
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
