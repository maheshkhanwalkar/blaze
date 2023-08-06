package inode

import "fmt"

type InodeType int

const (
	File InodeType = iota
	Directory
)

// Inode contains common inode data.
type Inode struct {
	Ino            int64 `json:"ino"`      // inode number
	VersionCounter int   `json:"vCounter"` // version counter
}

// EntryKey is the key structure for looking up a directory entry within a directory inode.
type EntryKey struct {
	Name    string `json:"name"`    // directory entry name
	Version int    `json:"version"` // version number
}

// DirEntry is a versioned, directory entry which points to a single version of an inode
type DirEntry struct {
	Ino     int64     `json:"ino"`     // corresponding inode number
	Itype   InodeType `json:"itype"`   // corresponding inode type
	Version int       `json:"version"` // corresponding inode version
}

func (key EntryKey) Key() string {
	return fmt.Sprintf("(%s,%d)", key.Name, key.Version)
}

func getAndIncrement(counter *int) int {
	version := *counter
	*counter++
	return version
}
