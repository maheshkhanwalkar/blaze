package core

import (
	"blaze/file"
	"crypto/sha256"
	"fmt"
	"os"
)

// Blob represents (as the name implies) a tracked object within the  version control system.
// The object is a snapshot in time of some file within the repository
type Blob struct {
	name string
	data []byte
}

// CreateBlob creates a new blob from the given data
func CreateBlob(data []byte) *Blob {
	name := generateName(data)
	return &Blob{name: name, data: data}
}

// LoadBlob loads the blob from disk
func LoadBlob(name string) *Blob {
	path := fmt.Sprintf(".blaze/object/%s", name)
	buffer := file.LoadBinaryFile(path)

	hash := computeHash(buffer)

	if hash != name {
		fmt.Printf("object tampering detected: %s has been altered", name)
		os.Exit(1)
	}

	return &Blob{name: name, data: buffer}
}

// ToDisk serializes the blob to disk
func (blob *Blob) ToDisk() {
	path := fmt.Sprintf(".blaze/object/%s", blob.name)
	f, err := os.Create(path)
	check(err)

	// Only need to write the data to disk, since the object name is encoded
	// within the file name
	_, err = f.Write(blob.data)
	check(err)
	err = f.Close()
	check(err)
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
