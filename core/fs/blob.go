package fs

import (
	"blaze/proto/fs"
	"crypto/sha256"
	"fmt"
)

// CreateBlob creates a new blob from the given data
func CreateBlob(data []byte) *fs.Blob {
	name := generateName(data)
	return &fs.Blob{Name: name, Data: data}
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
