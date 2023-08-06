package fs

import (
	"encoding/json"
	"fmt"
	"os"
)

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

// FileInode is a structure of a versioned inode that represents a file.
// Since this inode supports versioning, it maintains a mapping of a version to
// the underlying blob in the version-control filesystem.
type FileInode struct {
	Inode                     // common inode data
	VersionMap map[int]string `json:"versionMap"` // mapping of version number to blob
}

// DirInode is a structure of a versioned inode that represents a directory.
// Since this inode supports versioning it maintains a directory entry mapping that
// is keyed by version (as well as name) to get the corresponding versioned inode.
type DirInode struct {
	Inode                         // common inode data
	Entries map[EntryKey]DirEntry `json:"entries"` // directory entries
	dirty   bool
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

func (ip *FileInode) Update(blob string) int {
	version := getAndIncrement(&ip.VersionCounter)
	ip.VersionMap[version] = blob
	return version
}

func (ip *DirInode) MarshalJSON() ([]byte, error) {
	rekeyed := make(map[string]DirEntry)

	for key, value := range ip.Entries {
		nKey := fmt.Sprintf("(%s,%d)", key.Name, key.Version)
		rekeyed[nKey] = value
	}

	return json.Marshal(&struct {
		Inode
		Entries map[string]DirEntry
	}{
		Inode:   Inode{Ino: ip.Ino, VersionCounter: ip.VersionCounter},
		Entries: rekeyed,
	})
}

func (ip *DirInode) ToDisk() {
	// Nothing to write, if inode isn't dirty
	if !ip.dirty {
		return
	}

	path := fmt.Sprintf(".blaze/inode/%d", ip.Ino)
	f, err := os.Create(path)
	check(err)
	defer close(f)

	encoder := json.NewEncoder(f)
	err = encoder.Encode(ip)
	check(err)
}

func getAndIncrement(counter *int) int {
	version := *counter
	*counter++
	return version
}
