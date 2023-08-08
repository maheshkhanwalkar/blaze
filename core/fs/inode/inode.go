package inode

import "fmt"

// EntryKey is the key structure for looking up a directory entry within a directory inode.
type EntryKey struct {
	Name    string `json:"name"`    // directory entry name
	Version int    `json:"version"` // version number
}

func (key EntryKey) Key() string {
	return fmt.Sprintf("(%s,%d)", key.Name, key.Version)
}

func getAndIncrement(counter *int32) int32 {
	version := *counter
	*counter++
	return version
}
